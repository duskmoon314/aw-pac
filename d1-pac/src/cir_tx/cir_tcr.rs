#[doc = "Register `cir_tcr` reader"]
pub struct R(crate::R<CIR_TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cir_tcr` writer"]
pub struct W(crate::W<CIR_TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_TCR_SPEC>;
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
impl From<crate::W<CIR_TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tts` reader - Type of the transmission signal"]
pub type TTS_R = crate::BitReader<TTS_A>;
#[doc = "Type of the transmission signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TTS_A {
    #[doc = "0: The transmitting wave is a single non-cyclical pulse."]
    NON_CYCLICAL = 0,
    #[doc = "1: The transmitting wave is a cyclical short-pulse."]
    CYCLICAL = 1,
}
impl From<TTS_A> for bool {
    #[inline(always)]
    fn from(variant: TTS_A) -> Self {
        variant as u8 != 0
    }
}
impl TTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TTS_A {
        match self.bits {
            false => TTS_A::NON_CYCLICAL,
            true => TTS_A::CYCLICAL,
        }
    }
    #[doc = "Checks if the value of the field is `NON_CYCLICAL`"]
    #[inline(always)]
    pub fn is_non_cyclical(&self) -> bool {
        *self == TTS_A::NON_CYCLICAL
    }
    #[doc = "Checks if the value of the field is `CYCLICAL`"]
    #[inline(always)]
    pub fn is_cyclical(&self) -> bool {
        *self == TTS_A::CYCLICAL
    }
}
#[doc = "Field `tts` writer - Type of the transmission signal"]
pub type TTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_TCR_SPEC, TTS_A, O>;
impl<'a, const O: u8> TTS_W<'a, O> {
    #[doc = "The transmitting wave is a single non-cyclical pulse."]
    #[inline(always)]
    pub fn non_cyclical(self) -> &'a mut W {
        self.variant(TTS_A::NON_CYCLICAL)
    }
    #[doc = "The transmitting wave is a cyclical short-pulse."]
    #[inline(always)]
    pub fn cyclical(self) -> &'a mut W {
        self.variant(TTS_A::CYCLICAL)
    }
}
#[doc = "Field `rcs` reader - Reference Clock Select for CIR Transmit\n\nThe data in TX_FIFO is used to describe the pulse in Run-Length Code. The basic unit of pulse width is Reference Clock."]
pub type RCS_R = crate::FieldReader<u8, RCS_A>;
#[doc = "Reference Clock Select for CIR Transmit\n\nThe data in TX_FIFO is used to describe the pulse in Run-Length Code. The basic unit of pulse width is Reference Clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RCS_A {
    #[doc = "0: CIR Transmit reference clock is ir_clk"]
    IR_CLK = 0,
    #[doc = "1: CIR Transmit reference clock is ir_clk/2"]
    IR_CLK_2 = 1,
    #[doc = "2: CIR Transmit reference clock is ir_clk/4"]
    IR_CLK_4 = 2,
    #[doc = "3: CIR Transmit reference clock is ir_clk/8"]
    IR_CLK_8 = 3,
    #[doc = "4: CIR Transmit reference clock is ir_clk/64"]
    IR_CLK_64 = 4,
    #[doc = "5: CIR Transmit reference clock is ir_clk/128"]
    IR_CLK_128 = 5,
    #[doc = "6: CIR Transmit reference clock is ir_clk/256"]
    IR_CLK_256 = 6,
    #[doc = "7: CIR Transmit reference clock is ir_clk/512"]
    IR_CLK_512 = 7,
}
impl From<RCS_A> for u8 {
    #[inline(always)]
    fn from(variant: RCS_A) -> Self {
        variant as _
    }
}
impl RCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCS_A {
        match self.bits {
            0 => RCS_A::IR_CLK,
            1 => RCS_A::IR_CLK_2,
            2 => RCS_A::IR_CLK_4,
            3 => RCS_A::IR_CLK_8,
            4 => RCS_A::IR_CLK_64,
            5 => RCS_A::IR_CLK_128,
            6 => RCS_A::IR_CLK_256,
            7 => RCS_A::IR_CLK_512,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IR_CLK`"]
    #[inline(always)]
    pub fn is_ir_clk(&self) -> bool {
        *self == RCS_A::IR_CLK
    }
    #[doc = "Checks if the value of the field is `IR_CLK_2`"]
    #[inline(always)]
    pub fn is_ir_clk_2(&self) -> bool {
        *self == RCS_A::IR_CLK_2
    }
    #[doc = "Checks if the value of the field is `IR_CLK_4`"]
    #[inline(always)]
    pub fn is_ir_clk_4(&self) -> bool {
        *self == RCS_A::IR_CLK_4
    }
    #[doc = "Checks if the value of the field is `IR_CLK_8`"]
    #[inline(always)]
    pub fn is_ir_clk_8(&self) -> bool {
        *self == RCS_A::IR_CLK_8
    }
    #[doc = "Checks if the value of the field is `IR_CLK_64`"]
    #[inline(always)]
    pub fn is_ir_clk_64(&self) -> bool {
        *self == RCS_A::IR_CLK_64
    }
    #[doc = "Checks if the value of the field is `IR_CLK_128`"]
    #[inline(always)]
    pub fn is_ir_clk_128(&self) -> bool {
        *self == RCS_A::IR_CLK_128
    }
    #[doc = "Checks if the value of the field is `IR_CLK_256`"]
    #[inline(always)]
    pub fn is_ir_clk_256(&self) -> bool {
        *self == RCS_A::IR_CLK_256
    }
    #[doc = "Checks if the value of the field is `IR_CLK_512`"]
    #[inline(always)]
    pub fn is_ir_clk_512(&self) -> bool {
        *self == RCS_A::IR_CLK_512
    }
}
#[doc = "Field `rcs` writer - Reference Clock Select for CIR Transmit\n\nThe data in TX_FIFO is used to describe the pulse in Run-Length Code. The basic unit of pulse width is Reference Clock."]
pub type RCS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CIR_TCR_SPEC, u8, RCS_A, 3, O>;
impl<'a, const O: u8> RCS_W<'a, O> {
    #[doc = "CIR Transmit reference clock is ir_clk"]
    #[inline(always)]
    pub fn ir_clk(self) -> &'a mut W {
        self.variant(RCS_A::IR_CLK)
    }
    #[doc = "CIR Transmit reference clock is ir_clk/2"]
    #[inline(always)]
    pub fn ir_clk_2(self) -> &'a mut W {
        self.variant(RCS_A::IR_CLK_2)
    }
    #[doc = "CIR Transmit reference clock is ir_clk/4"]
    #[inline(always)]
    pub fn ir_clk_4(self) -> &'a mut W {
        self.variant(RCS_A::IR_CLK_4)
    }
    #[doc = "CIR Transmit reference clock is ir_clk/8"]
    #[inline(always)]
    pub fn ir_clk_8(self) -> &'a mut W {
        self.variant(RCS_A::IR_CLK_8)
    }
    #[doc = "CIR Transmit reference clock is ir_clk/64"]
    #[inline(always)]
    pub fn ir_clk_64(self) -> &'a mut W {
        self.variant(RCS_A::IR_CLK_64)
    }
    #[doc = "CIR Transmit reference clock is ir_clk/128"]
    #[inline(always)]
    pub fn ir_clk_128(self) -> &'a mut W {
        self.variant(RCS_A::IR_CLK_128)
    }
    #[doc = "CIR Transmit reference clock is ir_clk/256"]
    #[inline(always)]
    pub fn ir_clk_256(self) -> &'a mut W {
        self.variant(RCS_A::IR_CLK_256)
    }
    #[doc = "CIR Transmit reference clock is ir_clk/512"]
    #[inline(always)]
    pub fn ir_clk_512(self) -> &'a mut W {
        self.variant(RCS_A::IR_CLK_512)
    }
}
#[doc = "Field `css` reader - Cyclical Pulse Start/Stop Control"]
pub type CSS_R = crate::BitReader<CSS_A>;
#[doc = "Cyclical Pulse Start/Stop Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSS_A {
    #[doc = "0: Stop when cleared to '0'. From start to stop, all data in FIFO must be transmitted."]
    STOP = 0,
    #[doc = "1: Start. Start to transmit when it is set to '1'."]
    START = 1,
}
impl From<CSS_A> for bool {
    #[inline(always)]
    fn from(variant: CSS_A) -> Self {
        variant as u8 != 0
    }
}
impl CSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSS_A {
        match self.bits {
            false => CSS_A::STOP,
            true => CSS_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == CSS_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CSS_A::START
    }
}
#[doc = "Field `css` writer - Cyclical Pulse Start/Stop Control"]
pub type CSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_TCR_SPEC, CSS_A, O>;
impl<'a, const O: u8> CSS_W<'a, O> {
    #[doc = "Stop when cleared to '0'. From start to stop, all data in FIFO must be transmitted."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CSS_A::STOP)
    }
    #[doc = "Start. Start to transmit when it is set to '1'."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CSS_A::START)
    }
}
impl R {
    #[doc = "Bit 0 - Type of the transmission signal"]
    #[inline(always)]
    pub fn tts(&self) -> TTS_R {
        TTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reference Clock Select for CIR Transmit\n\nThe data in TX_FIFO is used to describe the pulse in Run-Length Code. The basic unit of pulse width is Reference Clock."]
    #[inline(always)]
    pub fn rcs(&self) -> RCS_R {
        RCS_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 7 - Cyclical Pulse Start/Stop Control"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Type of the transmission signal"]
    #[inline(always)]
    #[must_use]
    pub fn tts(&mut self) -> TTS_W<0> {
        TTS_W::new(self)
    }
    #[doc = "Bits 1:3 - Reference Clock Select for CIR Transmit\n\nThe data in TX_FIFO is used to describe the pulse in Run-Length Code. The basic unit of pulse width is Reference Clock."]
    #[inline(always)]
    #[must_use]
    pub fn rcs(&mut self) -> RCS_W<1> {
        RCS_W::new(self)
    }
    #[doc = "Bit 7 - Cyclical Pulse Start/Stop Control"]
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CSS_W<7> {
        CSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR Transmit Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_tcr](index.html) module"]
pub struct CIR_TCR_SPEC;
impl crate::RegisterSpec for CIR_TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_tcr::R](R) reader structure"]
impl crate::Readable for CIR_TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_tcr::W](W) writer structure"]
impl crate::Writable for CIR_TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_tcr to value 0"]
impl crate::Resettable for CIR_TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
