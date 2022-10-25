#[doc = "Register `prs_en` reader"]
pub struct R(crate::R<PRS_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRS_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRS_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRS_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `prs_en` writer"]
pub struct W(crate::W<PRS_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRS_EN_SPEC>;
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
impl From<crate::W<PRS_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRS_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `prs_en` reader - "]
pub type PRS_EN_R = crate::BitReader<PRS_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRS_EN_A {
    #[doc = "0: Reset and disable the parser module"]
    RESET_DISABLE = 0,
    #[doc = "1: Enable the parser module"]
    ENABLE = 1,
}
impl From<PRS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PRS_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PRS_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRS_EN_A {
        match self.bits {
            false => PRS_EN_A::RESET_DISABLE,
            true => PRS_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DISABLE`"]
    #[inline(always)]
    pub fn is_reset_disable(&self) -> bool {
        *self == PRS_EN_A::RESET_DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PRS_EN_A::ENABLE
    }
}
#[doc = "Field `prs_en` writer - "]
pub type PRS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS_EN_SPEC, PRS_EN_A, O>;
impl<'a, const O: u8> PRS_EN_W<'a, O> {
    #[doc = "Reset and disable the parser module"]
    #[inline(always)]
    pub fn reset_disable(self) -> &'a mut W {
        self.variant(PRS_EN_A::RESET_DISABLE)
    }
    #[doc = "Enable the parser module"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRS_EN_A::ENABLE)
    }
}
#[doc = "Field `prs_mode` reader - "]
pub type PRS_MODE_R = crate::BitReader<PRS_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRS_MODE_A {
    #[doc = "1: MCSI"]
    MCSI = 1,
}
impl From<PRS_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: PRS_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl PRS_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRS_MODE_A> {
        match self.bits {
            true => Some(PRS_MODE_A::MCSI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MCSI`"]
    #[inline(always)]
    pub fn is_mcsi(&self) -> bool {
        *self == PRS_MODE_A::MCSI
    }
}
#[doc = "Field `prs_mode` writer - "]
pub type PRS_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS_EN_SPEC, PRS_MODE_A, O>;
impl<'a, const O: u8> PRS_MODE_W<'a, O> {
    #[doc = "MCSI"]
    #[inline(always)]
    pub fn mcsi(self) -> &'a mut W {
        self.variant(PRS_MODE_A::MCSI)
    }
}
#[doc = "Field `prs_ch_mode` reader - "]
pub type PRS_CH_MODE_R = crate::BitReader<PRS_CH_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRS_CH_MODE_A {
    #[doc = "0: Parser output channel 0-3 corresponding from input channel 0-3"]
    CORRESPONDING = 0,
    #[doc = "1: Parser output channel 0-3 all from input channel 0 (MIPI SEHDR)"]
    ALL_FROM_0 = 1,
}
impl From<PRS_CH_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: PRS_CH_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl PRS_CH_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRS_CH_MODE_A {
        match self.bits {
            false => PRS_CH_MODE_A::CORRESPONDING,
            true => PRS_CH_MODE_A::ALL_FROM_0,
        }
    }
    #[doc = "Checks if the value of the field is `CORRESPONDING`"]
    #[inline(always)]
    pub fn is_corresponding(&self) -> bool {
        *self == PRS_CH_MODE_A::CORRESPONDING
    }
    #[doc = "Checks if the value of the field is `ALL_FROM_0`"]
    #[inline(always)]
    pub fn is_all_from_0(&self) -> bool {
        *self == PRS_CH_MODE_A::ALL_FROM_0
    }
}
#[doc = "Field `prs_ch_mode` writer - "]
pub type PRS_CH_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS_EN_SPEC, PRS_CH_MODE_A, O>;
impl<'a, const O: u8> PRS_CH_MODE_W<'a, O> {
    #[doc = "Parser output channel 0-3 corresponding from input channel 0-3"]
    #[inline(always)]
    pub fn corresponding(self) -> &'a mut W {
        self.variant(PRS_CH_MODE_A::CORRESPONDING)
    }
    #[doc = "Parser output channel 0-3 all from input channel 0 (MIPI SEHDR)"]
    #[inline(always)]
    pub fn all_from_0(self) -> &'a mut W {
        self.variant(PRS_CH_MODE_A::ALL_FROM_0)
    }
}
#[doc = "Field `pclk_en` reader - "]
pub type PCLK_EN_R = crate::BitReader<PCLK_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCLK_EN_A {
    #[doc = "0: Gate pclk input"]
    G_ATE = 0,
    #[doc = "1: Enable pclk input"]
    ENABLE = 1,
}
impl From<PCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK_EN_A {
        match self.bits {
            false => PCLK_EN_A::G_ATE,
            true => PCLK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `G_ATE`"]
    #[inline(always)]
    pub fn is_g_ate(&self) -> bool {
        *self == PCLK_EN_A::G_ATE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PCLK_EN_A::ENABLE
    }
}
#[doc = "Field `pclk_en` writer - "]
pub type PCLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS_EN_SPEC, PCLK_EN_A, O>;
impl<'a, const O: u8> PCLK_EN_W<'a, O> {
    #[doc = "Gate pclk input"]
    #[inline(always)]
    pub fn g_ate(self) -> &'a mut W {
        self.variant(PCLK_EN_A::G_ATE)
    }
    #[doc = "Enable pclk input"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PCLK_EN_A::ENABLE)
    }
}
#[doc = "Field `ncsic_en` reader - "]
pub type NCSIC_EN_R = crate::BitReader<NCSIC_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCSIC_EN_A {
    #[doc = "0: Reset and disable the NCSIC module"]
    RESET_DISABLE = 0,
    #[doc = "1: Enable the NCSIC module"]
    ENABLE = 1,
}
impl From<NCSIC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: NCSIC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl NCSIC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCSIC_EN_A {
        match self.bits {
            false => NCSIC_EN_A::RESET_DISABLE,
            true => NCSIC_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_DISABLE`"]
    #[inline(always)]
    pub fn is_reset_disable(&self) -> bool {
        *self == NCSIC_EN_A::RESET_DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NCSIC_EN_A::ENABLE
    }
}
#[doc = "Field `ncsic_en` writer - "]
pub type NCSIC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS_EN_SPEC, NCSIC_EN_A, O>;
impl<'a, const O: u8> NCSIC_EN_W<'a, O> {
    #[doc = "Reset and disable the NCSIC module"]
    #[inline(always)]
    pub fn reset_disable(self) -> &'a mut W {
        self.variant(NCSIC_EN_A::RESET_DISABLE)
    }
    #[doc = "Enable the NCSIC module"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(NCSIC_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn prs_en(&self) -> PRS_EN_R {
        PRS_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn prs_mode(&self) -> PRS_MODE_R {
        PRS_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn prs_ch_mode(&self) -> PRS_CH_MODE_R {
        PRS_CH_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pclk_en(&self) -> PCLK_EN_R {
        PCLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ncsic_en(&self) -> NCSIC_EN_R {
        NCSIC_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn prs_en(&mut self) -> PRS_EN_W<0> {
        PRS_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn prs_mode(&mut self) -> PRS_MODE_W<1> {
        PRS_MODE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn prs_ch_mode(&mut self) -> PRS_CH_MODE_W<2> {
        PRS_CH_MODE_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_en(&mut self) -> PCLK_EN_W<15> {
        PCLK_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ncsic_en(&mut self) -> NCSIC_EN_W<16> {
        NCSIC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parser Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prs_en](index.html) module"]
pub struct PRS_EN_SPEC;
impl crate::RegisterSpec for PRS_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prs_en::R](R) reader structure"]
impl crate::Readable for PRS_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prs_en::W](W) writer structure"]
impl crate::Writable for PRS_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets prs_en to value 0"]
impl crate::Resettable for PRS_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
