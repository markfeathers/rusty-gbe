use super::GBEmulator;

/*const LIGHTEST: [u8; 4] = [155, 188, 15, 255];
const LIGHT: [u8; 4] = [139, 172, 15, 255];
const DARK: [u8; 4] = [48, 98, 48, 255];
const DARKEST: [u8; 4] = [15, 56, 15, 255];*/
const LIGHTEST: [u8; 4] = [255, 255, 255, 255];
const LIGHT: [u8; 4] = [192, 192, 192, 255];
const DARK: [u8; 4] = [96, 96, 96, 255];
const DARKEST: [u8; 4] = [0, 0, 0, 255];

const LCDC: u16     = 0xFF40;
const STAT: u16     = 0xFF41;
const SCY: u16      = 0xFF42;
const SCX: u16      = 0xFF43;
const LY: u16       = 0xFF44;
const LYC: u16      = 0xFF45;
const DMA: u16      = 0xFF46;
const BGP: u16      = 0xFF47;
const OBP0: u16     = 0xFF48;
const OBP1: u16     = 0xFF49;
const WY: u16       = 0xFF4A;
const WX: u16       = 0xFF4B;

const VBLANK_SCANLINE: u8       = 144;
const VBLANK_SCANLINE_MAX: u8   = 153;
const GPU_CYCLES_PER_FRAME: u32 = 456;

impl GBEmulator {
    pub fn draw_scanline(&mut self) {
        let lcdc = self.mmu_read8(LCDC);

        if lcdc & (1 << 0) == 0 {
            /* LCD is off, should I draw a color across the frame here? */
            return;
        }
        
        let wy = self.mmu_read8(WY);
        let wx = self.mmu_read8(WX);
        let ly = self.mmu_read8(LY);
        let scy = self.mmu_read8(SCY);
        let scx = self.mmu_read8(SCX);

        /* Set to true if a window is enabled and visible */
        let draw_window: bool = if lcdc & (1 << 5) != 0 {
            wy <= ly
        } else {
            false
        };

        /* Some tile regions use signed or unsigned addresses */
        let (tile_offset, signed) = if lcdc & (1 << 4) != 0 {
            (0x8000, false)
        } else {
            (0x8800, true)
        };

        assert!(!signed);

        let bg_offset = if draw_window {
            if lcdc & (1 << 3) != 0 {
                0x9C00
            } else {
                0x9800
            }
        } else if lcdc & (1 << 6) != 0 {
            0x9C00
        } else {
            0x9800
        };

        let ypos = if draw_window {
            ly.wrapping_sub(wy)
        } else {
            ly.wrapping_add(scy)
        };

        let tile_row = ((ypos as u16)/8) * 32;
        for pixel in 0..160 {
            let xpos: u16 = if draw_window && (pixel as u16) >= (wx as u16) {
                //i.saturating_sub(wx.into())
                (pixel as u16) - (wx as u16)
            } else {
                (pixel as u16) + (scx as u16)
            };

            let tile_col = xpos / 8;
            let tile_addr: u16 = (bg_offset + tile_row + tile_col) as u16;
            let tile_num = self.mmu_read8(tile_addr) as u16;
            let tile_location: u16 = tile_offset + (tile_num * 16);
            let line = ((ypos as u16) % 8) * 2;

            let byte0 =  self.mmu_read8(tile_location + line);
            let byte1 =  self.mmu_read8(tile_location + line + 1);

            let bit = 7 - (xpos as u8) % 8;

            let mut value = (byte0 & bit) >> bit;
            value |= ((byte1 & bit) >> bit) << 1;

            let bgp = self.mmu_read8(BGP);

            let pixel_color = self.get_color_from_palette(value, bgp);
            
            //println!("ly {}, pixel {}", ly, pixel);
            let pixel_offset = ((144 * (ly as usize)) + pixel) * 4;

            //println!("pixel_offset {}", pixel_offset);
            self.framebuffer[pixel_offset] = pixel_color[0];
            self.framebuffer[pixel_offset + 1] = pixel_color[1];
            self.framebuffer[pixel_offset + 2] = pixel_color[2];
        }
    }

    fn get_color_from_palette(&self, value: u8, palette: u8) -> [u8; 4] {
        let shade = (palette >> (value * 2)) & 0x3;
        match shade {
            0b00 => LIGHTEST,
            0b01 => LIGHT,
            0b10 => DARK,
            0b11 => DARKEST,
            _ => panic!("Invalid tile pixel - value {:#04X} - shade {:#04X} - palette {:#04X}", value, shade, palette),
        }
    }

    pub fn gpu_run(&mut self, cycles: u32) {
        let lcdc = self.mmu_read8(LCDC);
        if lcdc & (1 << 7) == 0 { /* LCD Off */
            self.gpu_frame_cycles = GPU_CYCLES_PER_FRAME;
            self.mmu_write8(LY, 0x0);
            self.mmu_write8(STAT,0x1);
            return;
        }

        self.set_lcd_stat();

        let (result, overflow) = self.gpu_frame_cycles.overflowing_sub(cycles);
        if overflow {
            self.gpu_frame_cycles = 0;
            let mut ly = self.mmu_read8(LY);
            ly += 1;
            self.gpu_frame_cycles = GPU_CYCLES_PER_FRAME;

            if ly == VBLANK_SCANLINE {
                self.request_irq(0);
            } else if ly > VBLANK_SCANLINE_MAX {
                ly = 0;
            } else {
                self.draw_scanline();
            }

            self.mmu_write8(LY, ly);

        } else {
            self.gpu_frame_cycles = result;
        }
    }

    fn set_lcd_stat(&mut self) {
        let mut lcd_status = self.mmu_read8(STAT);
        let lcdc = self.mmu_read8(LCDC);
        let current_mode = lcdc & 0x3;
        let ly = self.mmu_read8(LY);

        /* Check LCD Status mode, if we are entering
         * a new mode with an IRQ enabled, request it */
        if ly > VBLANK_SCANLINE {
            /* VBLANK mode */
            if current_mode != 1  {
                lcd_status = (lcd_status & 0x3) | 0x1;
                if (current_mode != 1) && (lcd_status & (1 << 4) != 0) {
                    self.request_irq(1);
                }
            }
        } else {
            const MODE2_START: u32 = GPU_CYCLES_PER_FRAME - 80;
            const MODE3_START: u32 = MODE2_START - 172;

            if self.gpu_frame_cycles >= MODE2_START {
                /* OAM */
                lcd_status = (lcd_status & 0x3) | 0x2;
                if current_mode != 2 && (lcd_status & (1 << 5) != 0) {
                    self.request_irq(1);
                }
            } else if self.gpu_frame_cycles > MODE3_START {
                /* LCD */
                lcd_status = (lcd_status & 0x3) | 0x3;
            } else {
                /* HBLANK */
                lcd_status = lcd_status & 0x3;
                if current_mode != 3 && (lcd_status & (1 << 3) != 0) {
                    self.request_irq(1);
                }
            }
        }

        /* Coincidence IRQ */
        if ly == self.mmu_read8(LYC) {
            lcd_status |= 1 << 2;
            if lcd_status & (1 << 6) != 0 {
                self.request_irq(1);
            }
        } else {
            lcd_status &= !(1 << 2);
        }
        self.mmu_write8(STAT, lcd_status);
    }
}