#[doc = "Register `prs_ch%s_int_en` reader"]
pub struct R(crate::R<PRS_CH_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRS_CH_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRS_CH_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRS_CH_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `prs_ch%s_int_en` writer"]
pub struct W(crate::W<PRS_CH_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRS_CH_INT_EN_SPEC>;
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
impl From<crate::W<PRS_CH_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRS_CH_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `input_para_int_en[0-1]` reader - "]
pub type INPUT_PARA_INT_EN_R = crate::BitReader<INPUT_PARA_INT_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPUT_PARA_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<INPUT_PARA_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: INPUT_PARA_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl INPUT_PARA_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT_PARA_INT_EN_A {
        match self.bits {
            false => INPUT_PARA_INT_EN_A::DISABLE,
            true => INPUT_PARA_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INPUT_PARA_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INPUT_PARA_INT_EN_A::ENABLE
    }
}
#[doc = "Field `input_para_int_en[0-1]` writer - "]
pub type INPUT_PARA_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRS_CH_INT_EN_SPEC, INPUT_PARA_INT_EN_A, O>;
impl<'a, const O: u8> INPUT_PARA_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INPUT_PARA_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INPUT_PARA_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `mul_err_int_en` reader - Multi-channel writing error\n\nIndicates error has been detected for writing data to a wrong channel"]
pub type MUL_ERR_INT_EN_R = crate::BitReader<MUL_ERR_INT_EN_A>;
#[doc = "Multi-channel writing error\n\nIndicates error has been detected for writing data to a wrong channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUL_ERR_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<MUL_ERR_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MUL_ERR_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl MUL_ERR_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUL_ERR_INT_EN_A {
        match self.bits {
            false => MUL_ERR_INT_EN_A::DISABLE,
            true => MUL_ERR_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MUL_ERR_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MUL_ERR_INT_EN_A::ENABLE
    }
}
#[doc = "Field `mul_err_int_en` writer - Multi-channel writing error\n\nIndicates error has been detected for writing data to a wrong channel"]
pub type MUL_ERR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRS_CH_INT_EN_SPEC, MUL_ERR_INT_EN_A, O>;
impl<'a, const O: u8> MUL_ERR_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MUL_ERR_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MUL_ERR_INT_EN_A::ENABLE)
    }
}
impl R {
    #[doc = ""]
    #[inline(always)]
    pub unsafe fn input_para_int_en(&self, n: u8) -> INPUT_PARA_INT_EN_R {
        INPUT_PARA_INT_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn input_para0_int_en(&self) -> INPUT_PARA_INT_EN_R {
        INPUT_PARA_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn input_para1_int_en(&self) -> INPUT_PARA_INT_EN_R {
        INPUT_PARA_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Multi-channel writing error\n\nIndicates error has been detected for writing data to a wrong channel"]
    #[inline(always)]
    pub fn mul_err_int_en(&self) -> MUL_ERR_INT_EN_R {
        MUL_ERR_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = ""]
    #[inline(always)]
    #[must_use]
    pub unsafe fn input_para_int_en<const O: u8>(&mut self) -> INPUT_PARA_INT_EN_W<O> {
        INPUT_PARA_INT_EN_W::new(self)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn input_para0_int_en(&mut self) -> INPUT_PARA_INT_EN_W<0> {
        INPUT_PARA_INT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn input_para1_int_en(&mut self) -> INPUT_PARA_INT_EN_W<1> {
        INPUT_PARA_INT_EN_W::new(self)
    }
    #[doc = "Bit 2 - Multi-channel writing error\n\nIndicates error has been detected for writing data to a wrong channel"]
    #[inline(always)]
    #[must_use]
    pub fn mul_err_int_en(&mut self) -> MUL_ERR_INT_EN_W<2> {
        MUL_ERR_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parser Channel\\[i\\] Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prs_ch_int_en](index.html) module"]
pub struct PRS_CH_INT_EN_SPEC;
impl crate::RegisterSpec for PRS_CH_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prs_ch_int_en::R](R) reader structure"]
impl crate::Readable for PRS_CH_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prs_ch_int_en::W](W) writer structure"]
impl crate::Writable for PRS_CH_INT_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets prs_ch%s_int_en to value 0"]
impl crate::Resettable for PRS_CH_INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
