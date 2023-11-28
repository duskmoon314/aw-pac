#[doc = "Register `lcd_cpu_if` reader"]
pub type R = crate::R<LCD_CPU_IF_SPEC>;
#[doc = "Register `lcd_cpu_if` writer"]
pub type W = crate::W<LCD_CPU_IF_SPEC>;
#[doc = "Field `tri_en` reader - Enable trigger mode"]
pub type TRI_EN_R = crate::BitReader<TRI_EN_A>;
#[doc = "Enable trigger mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRI_EN_A {
    #[doc = "0: Trigger mode disable"]
    DISABLE = 0,
    #[doc = "1: Trigger mode enable"]
    ENABLE = 1,
}
impl From<TRI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TRI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TRI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRI_EN_A {
        match self.bits {
            false => TRI_EN_A::DISABLE,
            true => TRI_EN_A::ENABLE,
        }
    }
    #[doc = "Trigger mode disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TRI_EN_A::DISABLE
    }
    #[doc = "Trigger mode enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TRI_EN_A::ENABLE
    }
}
#[doc = "Field `tri_en` writer - Enable trigger mode"]
pub type TRI_EN_W<'a, REG> = crate::BitWriter<'a, REG, TRI_EN_A>;
impl<'a, REG> TRI_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TRI_EN_A::DISABLE)
    }
    #[doc = "Trigger mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TRI_EN_A::ENABLE)
    }
}
#[doc = "Field `tri_start` reader - Software must make sure that write '1' only when this flag is '0'.\n\nWriting '1' starts a frame flush and writing '0' has no effect.\n\nThis flag indicates the frame flush is running."]
pub type TRI_START_R = crate::BitReader;
#[doc = "Field `tri_start` writer - Software must make sure that write '1' only when this flag is '0'.\n\nWriting '1' starts a frame flush and writing '0' has no effect.\n\nThis flag indicates the frame flush is running."]
pub type TRI_START_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `tri_fifo_en` reader - Enable the trigger FIFO"]
pub type TRI_FIFO_EN_R = crate::BitReader<TRI_FIFO_EN_A>;
#[doc = "Enable the trigger FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRI_FIFO_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<TRI_FIFO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TRI_FIFO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TRI_FIFO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRI_FIFO_EN_A {
        match self.bits {
            false => TRI_FIFO_EN_A::DISABLE,
            true => TRI_FIFO_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TRI_FIFO_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TRI_FIFO_EN_A::ENABLE
    }
}
#[doc = "Field `tri_fifo_en` writer - Enable the trigger FIFO"]
pub type TRI_FIFO_EN_W<'a, REG> = crate::BitWriter<'a, REG, TRI_FIFO_EN_A>;
impl<'a, REG> TRI_FIFO_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TRI_FIFO_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TRI_FIFO_EN_A::ENABLE)
    }
}
#[doc = "Field `tri_fifo_bist_en` reader - Entry address is 0xFF8"]
pub type TRI_FIFO_BIST_EN_R = crate::BitReader<TRI_FIFO_BIST_EN_A>;
#[doc = "Entry address is 0xFF8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRI_FIFO_BIST_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<TRI_FIFO_BIST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TRI_FIFO_BIST_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TRI_FIFO_BIST_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRI_FIFO_BIST_EN_A {
        match self.bits {
            false => TRI_FIFO_BIST_EN_A::DISABLE,
            true => TRI_FIFO_BIST_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TRI_FIFO_BIST_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TRI_FIFO_BIST_EN_A::ENABLE
    }
}
#[doc = "Field `tri_fifo_bist_en` writer - Entry address is 0xFF8"]
pub type TRI_FIFO_BIST_EN_W<'a, REG> = crate::BitWriter<'a, REG, TRI_FIFO_BIST_EN_A>;
impl<'a, REG> TRI_FIFO_BIST_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TRI_FIFO_BIST_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TRI_FIFO_BIST_EN_A::ENABLE)
    }
}
#[doc = "Field `flush` reader - Direct transfer mode\n\nIf it is enabled, FIFO1 is regardless of the HV timing, the pixels data keep being transferred unless the input FIFO was empty.\n\nData output rate is controlled by DCLK."]
pub type FLUSH_R = crate::BitReader;
#[doc = "Field `flush` writer - Direct transfer mode\n\nIf it is enabled, FIFO1 is regardless of the HV timing, the pixels data keep being transferred unless the input FIFO was empty.\n\nData output rate is controlled by DCLK."]
pub type FLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `auto` reader - Auto transfer mode\n\nIf it is 1, all the valid data during this frame are written to panel.\n\nNote: This bit is sampled by Vsync."]
pub type AUTO_R = crate::BitReader;
#[doc = "Field `auto` writer - Auto transfer mode\n\nIf it is 1, all the valid data during this frame are written to panel.\n\nNote: This bit is sampled by Vsync."]
pub type AUTO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_flag` reader - The status of read operation."]
pub type RD_FLAG_R = crate::BitReader<RD_FLAG_A>;
#[doc = "The status of read operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RD_FLAG_A {
    #[doc = "0: Read operation is finishing"]
    FINISHING = 0,
    #[doc = "1: Read operation is pending"]
    PENDING = 1,
}
impl From<RD_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: RD_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl RD_FLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RD_FLAG_A {
        match self.bits {
            false => RD_FLAG_A::FINISHING,
            true => RD_FLAG_A::PENDING,
        }
    }
    #[doc = "Read operation is finishing"]
    #[inline(always)]
    pub fn is_finishing(&self) -> bool {
        *self == RD_FLAG_A::FINISHING
    }
    #[doc = "Read operation is pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RD_FLAG_A::PENDING
    }
}
#[doc = "Field `wr_flag` reader - The status of write operation."]
pub type WR_FLAG_R = crate::BitReader<WR_FLAG_A>;
#[doc = "The status of write operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WR_FLAG_A {
    #[doc = "0: Write operation is finishing"]
    FINISHING = 0,
    #[doc = "1: Write operation is pending"]
    PENDING = 1,
}
impl From<WR_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: WR_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl WR_FLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WR_FLAG_A {
        match self.bits {
            false => WR_FLAG_A::FINISHING,
            true => WR_FLAG_A::PENDING,
        }
    }
    #[doc = "Write operation is finishing"]
    #[inline(always)]
    pub fn is_finishing(&self) -> bool {
        *self == WR_FLAG_A::FINISHING
    }
    #[doc = "Write operation is pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == WR_FLAG_A::PENDING
    }
}
#[doc = "Field `ca` reader - Pin A1 value in 8080 mode WR/RD execute"]
pub type CA_R = crate::BitReader;
#[doc = "Field `ca` writer - Pin A1 value in 8080 mode WR/RD execute"]
pub type CA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `da` reader - Pin A1 value in 8080 mode auto/flash states"]
pub type DA_R = crate::BitReader;
#[doc = "Field `da` writer - Pin A1 value in 8080 mode auto/flash states"]
pub type DA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpu_mode` reader - Set the cpu interface work mode"]
pub type CPU_MODE_R = crate::FieldReader<CPU_MODE_A>;
#[doc = "Set the cpu interface work mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPU_MODE_A {
    #[doc = "0: 18-bit/256K mode"]
    B_IT18_256K_MODE = 0,
    #[doc = "2: 16-bit mode0"]
    B_IT16_MODE0 = 2,
    #[doc = "4: 16-bit mode1"]
    B_IT16_MODE1 = 4,
    #[doc = "6: 16-bit mode2"]
    B_IT16_MODE2 = 6,
    #[doc = "8: 16-bit mode3"]
    B_IT16_MODE3 = 8,
    #[doc = "10: 9-bit mode"]
    B_IT9_MODE = 10,
    #[doc = "12: 8-bit 256K mode"]
    B_IT8_256K_MODE = 12,
    #[doc = "14: 8-bit 65K mode"]
    B_IT8_65K_MODE = 14,
    #[doc = "1: 24-bit for DSI"]
    B_IT24_FOR_DSI = 1,
}
impl From<CPU_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CPU_MODE_A {
    type Ux = u8;
}
impl CPU_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CPU_MODE_A> {
        match self.bits {
            0 => Some(CPU_MODE_A::B_IT18_256K_MODE),
            2 => Some(CPU_MODE_A::B_IT16_MODE0),
            4 => Some(CPU_MODE_A::B_IT16_MODE1),
            6 => Some(CPU_MODE_A::B_IT16_MODE2),
            8 => Some(CPU_MODE_A::B_IT16_MODE3),
            10 => Some(CPU_MODE_A::B_IT9_MODE),
            12 => Some(CPU_MODE_A::B_IT8_256K_MODE),
            14 => Some(CPU_MODE_A::B_IT8_65K_MODE),
            1 => Some(CPU_MODE_A::B_IT24_FOR_DSI),
            _ => None,
        }
    }
    #[doc = "18-bit/256K mode"]
    #[inline(always)]
    pub fn is_b_it18_256k_mode(&self) -> bool {
        *self == CPU_MODE_A::B_IT18_256K_MODE
    }
    #[doc = "16-bit mode0"]
    #[inline(always)]
    pub fn is_b_it16_mode0(&self) -> bool {
        *self == CPU_MODE_A::B_IT16_MODE0
    }
    #[doc = "16-bit mode1"]
    #[inline(always)]
    pub fn is_b_it16_mode1(&self) -> bool {
        *self == CPU_MODE_A::B_IT16_MODE1
    }
    #[doc = "16-bit mode2"]
    #[inline(always)]
    pub fn is_b_it16_mode2(&self) -> bool {
        *self == CPU_MODE_A::B_IT16_MODE2
    }
    #[doc = "16-bit mode3"]
    #[inline(always)]
    pub fn is_b_it16_mode3(&self) -> bool {
        *self == CPU_MODE_A::B_IT16_MODE3
    }
    #[doc = "9-bit mode"]
    #[inline(always)]
    pub fn is_b_it9_mode(&self) -> bool {
        *self == CPU_MODE_A::B_IT9_MODE
    }
    #[doc = "8-bit 256K mode"]
    #[inline(always)]
    pub fn is_b_it8_256k_mode(&self) -> bool {
        *self == CPU_MODE_A::B_IT8_256K_MODE
    }
    #[doc = "8-bit 65K mode"]
    #[inline(always)]
    pub fn is_b_it8_65k_mode(&self) -> bool {
        *self == CPU_MODE_A::B_IT8_65K_MODE
    }
    #[doc = "24-bit for DSI"]
    #[inline(always)]
    pub fn is_b_it24_for_dsi(&self) -> bool {
        *self == CPU_MODE_A::B_IT24_FOR_DSI
    }
}
#[doc = "Field `cpu_mode` writer - Set the cpu interface work mode"]
pub type CPU_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CPU_MODE_A>;
impl<'a, REG> CPU_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "18-bit/256K mode"]
    #[inline(always)]
    pub fn b_it18_256k_mode(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_MODE_A::B_IT18_256K_MODE)
    }
    #[doc = "16-bit mode0"]
    #[inline(always)]
    pub fn b_it16_mode0(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_MODE_A::B_IT16_MODE0)
    }
    #[doc = "16-bit mode1"]
    #[inline(always)]
    pub fn b_it16_mode1(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_MODE_A::B_IT16_MODE1)
    }
    #[doc = "16-bit mode2"]
    #[inline(always)]
    pub fn b_it16_mode2(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_MODE_A::B_IT16_MODE2)
    }
    #[doc = "16-bit mode3"]
    #[inline(always)]
    pub fn b_it16_mode3(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_MODE_A::B_IT16_MODE3)
    }
    #[doc = "9-bit mode"]
    #[inline(always)]
    pub fn b_it9_mode(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_MODE_A::B_IT9_MODE)
    }
    #[doc = "8-bit 256K mode"]
    #[inline(always)]
    pub fn b_it8_256k_mode(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_MODE_A::B_IT8_256K_MODE)
    }
    #[doc = "8-bit 65K mode"]
    #[inline(always)]
    pub fn b_it8_65k_mode(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_MODE_A::B_IT8_65K_MODE)
    }
    #[doc = "24-bit for DSI"]
    #[inline(always)]
    pub fn b_it24_for_dsi(self) -> &'a mut crate::W<REG> {
        self.variant(CPU_MODE_A::B_IT24_FOR_DSI)
    }
}
impl R {
    #[doc = "Bit 0 - Enable trigger mode"]
    #[inline(always)]
    pub fn tri_en(&self) -> TRI_EN_R {
        TRI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software must make sure that write '1' only when this flag is '0'.\n\nWriting '1' starts a frame flush and writing '0' has no effect.\n\nThis flag indicates the frame flush is running."]
    #[inline(always)]
    pub fn tri_start(&self) -> TRI_START_R {
        TRI_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the trigger FIFO"]
    #[inline(always)]
    pub fn tri_fifo_en(&self) -> TRI_FIFO_EN_R {
        TRI_FIFO_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Entry address is 0xFF8"]
    #[inline(always)]
    pub fn tri_fifo_bist_en(&self) -> TRI_FIFO_BIST_EN_R {
        TRI_FIFO_BIST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Direct transfer mode\n\nIf it is enabled, FIFO1 is regardless of the HV timing, the pixels data keep being transferred unless the input FIFO was empty.\n\nData output rate is controlled by DCLK."]
    #[inline(always)]
    pub fn flush(&self) -> FLUSH_R {
        FLUSH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Auto transfer mode\n\nIf it is 1, all the valid data during this frame are written to panel.\n\nNote: This bit is sampled by Vsync."]
    #[inline(always)]
    pub fn auto(&self) -> AUTO_R {
        AUTO_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 22 - The status of read operation."]
    #[inline(always)]
    pub fn rd_flag(&self) -> RD_FLAG_R {
        RD_FLAG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The status of write operation."]
    #[inline(always)]
    pub fn wr_flag(&self) -> WR_FLAG_R {
        WR_FLAG_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Pin A1 value in 8080 mode WR/RD execute"]
    #[inline(always)]
    pub fn ca(&self) -> CA_R {
        CA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pin A1 value in 8080 mode auto/flash states"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Set the cpu interface work mode"]
    #[inline(always)]
    pub fn cpu_mode(&self) -> CPU_MODE_R {
        CPU_MODE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable trigger mode"]
    #[inline(always)]
    #[must_use]
    pub fn tri_en(&mut self) -> TRI_EN_W<LCD_CPU_IF_SPEC> {
        TRI_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software must make sure that write '1' only when this flag is '0'.\n\nWriting '1' starts a frame flush and writing '0' has no effect.\n\nThis flag indicates the frame flush is running."]
    #[inline(always)]
    #[must_use]
    pub fn tri_start(&mut self) -> TRI_START_W<LCD_CPU_IF_SPEC> {
        TRI_START_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable the trigger FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn tri_fifo_en(&mut self) -> TRI_FIFO_EN_W<LCD_CPU_IF_SPEC> {
        TRI_FIFO_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Entry address is 0xFF8"]
    #[inline(always)]
    #[must_use]
    pub fn tri_fifo_bist_en(&mut self) -> TRI_FIFO_BIST_EN_W<LCD_CPU_IF_SPEC> {
        TRI_FIFO_BIST_EN_W::new(self, 3)
    }
    #[doc = "Bit 16 - Direct transfer mode\n\nIf it is enabled, FIFO1 is regardless of the HV timing, the pixels data keep being transferred unless the input FIFO was empty.\n\nData output rate is controlled by DCLK."]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FLUSH_W<LCD_CPU_IF_SPEC> {
        FLUSH_W::new(self, 16)
    }
    #[doc = "Bit 17 - Auto transfer mode\n\nIf it is 1, all the valid data during this frame are written to panel.\n\nNote: This bit is sampled by Vsync."]
    #[inline(always)]
    #[must_use]
    pub fn auto(&mut self) -> AUTO_W<LCD_CPU_IF_SPEC> {
        AUTO_W::new(self, 17)
    }
    #[doc = "Bit 25 - Pin A1 value in 8080 mode WR/RD execute"]
    #[inline(always)]
    #[must_use]
    pub fn ca(&mut self) -> CA_W<LCD_CPU_IF_SPEC> {
        CA_W::new(self, 25)
    }
    #[doc = "Bit 26 - Pin A1 value in 8080 mode auto/flash states"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<LCD_CPU_IF_SPEC> {
        DA_W::new(self, 26)
    }
    #[doc = "Bits 28:31 - Set the cpu interface work mode"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_mode(&mut self) -> CPU_MODE_W<LCD_CPU_IF_SPEC> {
        CPU_MODE_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LCD CPU Panel Interface Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_if::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_if::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CPU_IF_SPEC;
impl crate::RegisterSpec for LCD_CPU_IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_cpu_if::R`](R) reader structure"]
impl crate::Readable for LCD_CPU_IF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_cpu_if::W`](W) writer structure"]
impl crate::Writable for LCD_CPU_IF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x02;
}
#[doc = "`reset()` method sets lcd_cpu_if to value 0"]
impl crate::Resettable for LCD_CPU_IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
