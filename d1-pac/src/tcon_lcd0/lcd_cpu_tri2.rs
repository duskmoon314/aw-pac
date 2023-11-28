#[doc = "Register `lcd_cpu_tri2` reader"]
pub type R = crate::R<LCD_CPU_TRI2_SPEC>;
#[doc = "Register `lcd_cpu_tri2` writer"]
pub type W = crate::W<LCD_CPU_TRI2_SPEC>;
#[doc = "Field `trans_start_set` reader - Usual set as the length of a line."]
pub type TRANS_START_SET_R = crate::FieldReader<u16>;
#[doc = "Field `trans_start_set` writer - Usual set as the length of a line."]
pub type TRANS_START_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `sync_mode` reader - Set the sync mode in CPU interface."]
pub type SYNC_MODE_R = crate::FieldReader;
#[doc = "Field `sync_mode` writer - Set the sync mode in CPU interface."]
pub type SYNC_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `trans_start_mode` reader - Select the FIFOs used in CPU mode."]
pub type TRANS_START_MODE_R = crate::BitReader<TRANS_START_MODE_A>;
#[doc = "Select the FIFOs used in CPU mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRANS_START_MODE_A {
    #[doc = "0: ECC_FIFO+TRI_FIFO"]
    ECC_FIFO_TRI_FIFO = 0,
    #[doc = "1: TRI_FIFO"]
    TRI_FIFO = 1,
}
impl From<TRANS_START_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: TRANS_START_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRANS_START_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRANS_START_MODE_A {
        match self.bits {
            false => TRANS_START_MODE_A::ECC_FIFO_TRI_FIFO,
            true => TRANS_START_MODE_A::TRI_FIFO,
        }
    }
    #[doc = "ECC_FIFO+TRI_FIFO"]
    #[inline(always)]
    pub fn is_ecc_fifo_tri_fifo(&self) -> bool {
        *self == TRANS_START_MODE_A::ECC_FIFO_TRI_FIFO
    }
    #[doc = "TRI_FIFO"]
    #[inline(always)]
    pub fn is_tri_fifo(&self) -> bool {
        *self == TRANS_START_MODE_A::TRI_FIFO
    }
}
#[doc = "Field `trans_start_mode` writer - Select the FIFOs used in CPU mode."]
pub type TRANS_START_MODE_W<'a, REG> = crate::BitWriter<'a, REG, TRANS_START_MODE_A>;
impl<'a, REG> TRANS_START_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECC_FIFO+TRI_FIFO"]
    #[inline(always)]
    pub fn ecc_fifo_tri_fifo(self) -> &'a mut crate::W<REG> {
        self.variant(TRANS_START_MODE_A::ECC_FIFO_TRI_FIFO)
    }
    #[doc = "TRI_FIFO"]
    #[inline(always)]
    pub fn tri_fifo(self) -> &'a mut crate::W<REG> {
        self.variant(TRANS_START_MODE_A::TRI_FIFO)
    }
}
#[doc = "Field `start_dly` reader - T_dly = (Start_Delay +1) * be_clk*8."]
pub type START_DLY_R = crate::FieldReader<u16>;
#[doc = "Field `start_dly` writer - T_dly = (Start_Delay +1) * be_clk*8."]
pub type START_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:12 - Usual set as the length of a line."]
    #[inline(always)]
    pub fn trans_start_set(&self) -> TRANS_START_SET_R {
        TRANS_START_SET_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:14 - Set the sync mode in CPU interface."]
    #[inline(always)]
    pub fn sync_mode(&self) -> SYNC_MODE_R {
        SYNC_MODE_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Select the FIFOs used in CPU mode."]
    #[inline(always)]
    pub fn trans_start_mode(&self) -> TRANS_START_MODE_R {
        TRANS_START_MODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - T_dly = (Start_Delay +1) * be_clk*8."]
    #[inline(always)]
    pub fn start_dly(&self) -> START_DLY_R {
        START_DLY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Usual set as the length of a line."]
    #[inline(always)]
    #[must_use]
    pub fn trans_start_set(&mut self) -> TRANS_START_SET_W<LCD_CPU_TRI2_SPEC> {
        TRANS_START_SET_W::new(self, 0)
    }
    #[doc = "Bits 13:14 - Set the sync mode in CPU interface."]
    #[inline(always)]
    #[must_use]
    pub fn sync_mode(&mut self) -> SYNC_MODE_W<LCD_CPU_TRI2_SPEC> {
        SYNC_MODE_W::new(self, 13)
    }
    #[doc = "Bit 15 - Select the FIFOs used in CPU mode."]
    #[inline(always)]
    #[must_use]
    pub fn trans_start_mode(&mut self) -> TRANS_START_MODE_W<LCD_CPU_TRI2_SPEC> {
        TRANS_START_MODE_W::new(self, 15)
    }
    #[doc = "Bits 16:31 - T_dly = (Start_Delay +1) * be_clk*8."]
    #[inline(always)]
    #[must_use]
    pub fn start_dly(&mut self) -> START_DLY_W<LCD_CPU_TRI2_SPEC> {
        START_DLY_W::new(self, 16)
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
#[doc = "LCD CPU Panel Trigger Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_tri2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_tri2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CPU_TRI2_SPEC;
impl crate::RegisterSpec for LCD_CPU_TRI2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_cpu_tri2::R`](R) reader structure"]
impl crate::Readable for LCD_CPU_TRI2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_cpu_tri2::W`](W) writer structure"]
impl crate::Writable for LCD_CPU_TRI2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cpu_tri2 to value 0"]
impl crate::Resettable for LCD_CPU_TRI2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
