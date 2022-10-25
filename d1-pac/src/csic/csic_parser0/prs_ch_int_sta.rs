#[doc = "Register `prs_ch%s_int_sta` reader"]
pub struct R(crate::R<PRS_CH_INT_STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRS_CH_INT_STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRS_CH_INT_STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRS_CH_INT_STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `prs_ch%s_int_sta` writer"]
pub struct W(crate::W<PRS_CH_INT_STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRS_CH_INT_STA_SPEC>;
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
impl From<crate::W<PRS_CH_INT_STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRS_CH_INT_STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `input_src_pd[0-1]` reader - "]
pub type INPUT_SRC_PD_R = crate::BitReader<INPUT_SRC_PD_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPUT_SRC_PD_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<INPUT_SRC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: INPUT_SRC_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl INPUT_SRC_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT_SRC_PD_A {
        match self.bits {
            false => INPUT_SRC_PD_A::NO_PENDING,
            true => INPUT_SRC_PD_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == INPUT_SRC_PD_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == INPUT_SRC_PD_A::PENDING
    }
}
#[doc = "Field `input_src_pd[0-1]` writer - "]
pub type INPUT_SRC_PD_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, PRS_CH_INT_STA_SPEC, INPUT_SRC_PD_A, O>;
impl<'a, const O: u8> INPUT_SRC_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(INPUT_SRC_PD_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(INPUT_SRC_PD_A::PENDING)
    }
}
#[doc = "Field `mul_err_pd` reader - Multi-channel writing error pending"]
pub type MUL_ERR_PD_R = crate::BitReader<MUL_ERR_PD_A>;
#[doc = "Multi-channel writing error pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUL_ERR_PD_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<MUL_ERR_PD_A> for bool {
    #[inline(always)]
    fn from(variant: MUL_ERR_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl MUL_ERR_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUL_ERR_PD_A {
        match self.bits {
            false => MUL_ERR_PD_A::NO_PENDING,
            true => MUL_ERR_PD_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == MUL_ERR_PD_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == MUL_ERR_PD_A::PENDING
    }
}
#[doc = "Field `mul_err_pd` writer - Multi-channel writing error pending"]
pub type MUL_ERR_PD_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, PRS_CH_INT_STA_SPEC, MUL_ERR_PD_A, O>;
impl<'a, const O: u8> MUL_ERR_PD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(MUL_ERR_PD_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(MUL_ERR_PD_A::PENDING)
    }
}
impl R {
    #[doc = ""]
    #[inline(always)]
    pub unsafe fn input_src_pd(&self, n: u8) -> INPUT_SRC_PD_R {
        INPUT_SRC_PD_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn input_src_pd0(&self) -> INPUT_SRC_PD_R {
        INPUT_SRC_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn input_src_pd1(&self) -> INPUT_SRC_PD_R {
        INPUT_SRC_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Multi-channel writing error pending"]
    #[inline(always)]
    pub fn mul_err_pd(&self) -> MUL_ERR_PD_R {
        MUL_ERR_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = ""]
    #[inline(always)]
    #[must_use]
    pub unsafe fn input_src_pd<const O: u8>(&mut self) -> INPUT_SRC_PD_W<O> {
        INPUT_SRC_PD_W::new(self)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn input_src_pd0(&mut self) -> INPUT_SRC_PD_W<0> {
        INPUT_SRC_PD_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn input_src_pd1(&mut self) -> INPUT_SRC_PD_W<1> {
        INPUT_SRC_PD_W::new(self)
    }
    #[doc = "Bit 2 - Multi-channel writing error pending"]
    #[inline(always)]
    #[must_use]
    pub fn mul_err_pd(&mut self) -> MUL_ERR_PD_W<2> {
        MUL_ERR_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parser Channel\\[i\\] Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prs_ch_int_sta](index.html) module"]
pub struct PRS_CH_INT_STA_SPEC;
impl crate::RegisterSpec for PRS_CH_INT_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prs_ch_int_sta::R](R) reader structure"]
impl crate::Readable for PRS_CH_INT_STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prs_ch_int_sta::W](W) writer structure"]
impl crate::Writable for PRS_CH_INT_STA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x05;
}
#[doc = "`reset()` method sets prs_ch%s_int_sta to value 0"]
impl crate::Resettable for PRS_CH_INT_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
