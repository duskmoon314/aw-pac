#[doc = "Register `lcd_cpu_tri2` reader"]
pub struct R(crate::R<LCD_CPU_TRI2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CPU_TRI2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CPU_TRI2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CPU_TRI2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_cpu_tri2` writer"]
pub struct W(crate::W<LCD_CPU_TRI2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CPU_TRI2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LCD_CPU_TRI2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CPU_TRI2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `trans_start_set` reader - Usual set as the length of a line."]
pub type TRANS_START_SET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `trans_start_set` writer - Usual set as the length of a line."]
pub type TRANS_START_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CPU_TRI2_SPEC, u16, u16, 13, O>;
#[doc = "Field `sync_mode` reader - Set the sync mode in CPU interface."]
pub type SYNC_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sync_mode` writer - Set the sync mode in CPU interface."]
pub type SYNC_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CPU_TRI2_SPEC, u8, u8, 2, O>;
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
    pub fn variant(&self) -> TRANS_START_MODE_A {
        match self.bits {
            false => TRANS_START_MODE_A::ECC_FIFO_TRI_FIFO,
            true => TRANS_START_MODE_A::TRI_FIFO,
        }
    }
    #[doc = "Checks if the value of the field is `ECC_FIFO_TRI_FIFO`"]
    #[inline(always)]
    pub fn is_ecc_fifo_tri_fifo(&self) -> bool {
        *self == TRANS_START_MODE_A::ECC_FIFO_TRI_FIFO
    }
    #[doc = "Checks if the value of the field is `TRI_FIFO`"]
    #[inline(always)]
    pub fn is_tri_fifo(&self) -> bool {
        *self == TRANS_START_MODE_A::TRI_FIFO
    }
}
#[doc = "Field `trans_start_mode` writer - Select the FIFOs used in CPU mode."]
pub type TRANS_START_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_CPU_TRI2_SPEC, TRANS_START_MODE_A, O>;
impl<'a, const O: u8> TRANS_START_MODE_W<'a, O> {
    #[doc = "ECC_FIFO+TRI_FIFO"]
    #[inline(always)]
    pub fn ecc_fifo_tri_fifo(self) -> &'a mut W {
        self.variant(TRANS_START_MODE_A::ECC_FIFO_TRI_FIFO)
    }
    #[doc = "TRI_FIFO"]
    #[inline(always)]
    pub fn tri_fifo(self) -> &'a mut W {
        self.variant(TRANS_START_MODE_A::TRI_FIFO)
    }
}
#[doc = "Field `start_dly` reader - T_dly = (Start_Delay +1) * be_clk*8."]
pub type START_DLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `start_dly` writer - T_dly = (Start_Delay +1) * be_clk*8."]
pub type START_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CPU_TRI2_SPEC, u16, u16, 16, O>;
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
    pub fn trans_start_set(&mut self) -> TRANS_START_SET_W<0> {
        TRANS_START_SET_W::new(self)
    }
    #[doc = "Bits 13:14 - Set the sync mode in CPU interface."]
    #[inline(always)]
    #[must_use]
    pub fn sync_mode(&mut self) -> SYNC_MODE_W<13> {
        SYNC_MODE_W::new(self)
    }
    #[doc = "Bit 15 - Select the FIFOs used in CPU mode."]
    #[inline(always)]
    #[must_use]
    pub fn trans_start_mode(&mut self) -> TRANS_START_MODE_W<15> {
        TRANS_START_MODE_W::new(self)
    }
    #[doc = "Bits 16:31 - T_dly = (Start_Delay +1) * be_clk*8."]
    #[inline(always)]
    #[must_use]
    pub fn start_dly(&mut self) -> START_DLY_W<16> {
        START_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD CPU Panel Trigger Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_cpu_tri2](index.html) module"]
pub struct LCD_CPU_TRI2_SPEC;
impl crate::RegisterSpec for LCD_CPU_TRI2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_cpu_tri2::R](R) reader structure"]
impl crate::Readable for LCD_CPU_TRI2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_cpu_tri2::W](W) writer structure"]
impl crate::Writable for LCD_CPU_TRI2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cpu_tri2 to value 0"]
impl crate::Resettable for LCD_CPU_TRI2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
