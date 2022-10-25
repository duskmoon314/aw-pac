#[doc = "Register `tve_clock_gating` reader"]
pub struct R(crate::R<TVE_CLOCK_GATING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_CLOCK_GATING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_CLOCK_GATING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_CLOCK_GATING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_clock_gating` writer"]
pub struct W(crate::W<TVE_CLOCK_GATING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_CLOCK_GATING_SPEC>;
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
impl From<crate::W<TVE_CLOCK_GATING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_CLOCK_GATING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tve_en` reader - Video Encoder enable, default disable, write 1 to take it out of the reset state"]
pub type TVE_EN_R = crate::BitReader<TVE_EN_A>;
#[doc = "Video Encoder enable, default disable, write 1 to take it out of the reset state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TVE_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<TVE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TVE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TVE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TVE_EN_A {
        match self.bits {
            false => TVE_EN_A::DISABLE,
            true => TVE_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TVE_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TVE_EN_A::ENABLE
    }
}
#[doc = "Field `tve_en` writer - Video Encoder enable, default disable, write 1 to take it out of the reset state"]
pub type TVE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TVE_CLOCK_GATING_SPEC, TVE_EN_A, O>;
impl<'a, const O: u8> TVE_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TVE_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TVE_EN_A::ENABLE)
    }
}
#[doc = "Field `upsample_for_cvbs` reader - "]
pub type UPSAMPLE_FOR_CVBS_R = crate::FieldReader<u8, UPSAMPLE_FOR_CVBS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPSAMPLE_FOR_CVBS_A {
    #[doc = "0: 27 MHz"]
    F27M = 0,
    #[doc = "1: 54 MHz"]
    F54M = 1,
    #[doc = "2: 108 MHz"]
    F108M = 2,
    #[doc = "3: 216 MHz"]
    F216M = 3,
}
impl From<UPSAMPLE_FOR_CVBS_A> for u8 {
    #[inline(always)]
    fn from(variant: UPSAMPLE_FOR_CVBS_A) -> Self {
        variant as _
    }
}
impl UPSAMPLE_FOR_CVBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPSAMPLE_FOR_CVBS_A {
        match self.bits {
            0 => UPSAMPLE_FOR_CVBS_A::F27M,
            1 => UPSAMPLE_FOR_CVBS_A::F54M,
            2 => UPSAMPLE_FOR_CVBS_A::F108M,
            3 => UPSAMPLE_FOR_CVBS_A::F216M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `F27M`"]
    #[inline(always)]
    pub fn is_f27m(&self) -> bool {
        *self == UPSAMPLE_FOR_CVBS_A::F27M
    }
    #[doc = "Checks if the value of the field is `F54M`"]
    #[inline(always)]
    pub fn is_f54m(&self) -> bool {
        *self == UPSAMPLE_FOR_CVBS_A::F54M
    }
    #[doc = "Checks if the value of the field is `F108M`"]
    #[inline(always)]
    pub fn is_f108m(&self) -> bool {
        *self == UPSAMPLE_FOR_CVBS_A::F108M
    }
    #[doc = "Checks if the value of the field is `F216M`"]
    #[inline(always)]
    pub fn is_f216m(&self) -> bool {
        *self == UPSAMPLE_FOR_CVBS_A::F216M
    }
}
#[doc = "Field `upsample_for_cvbs` writer - "]
pub type UPSAMPLE_FOR_CVBS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TVE_CLOCK_GATING_SPEC, u8, UPSAMPLE_FOR_CVBS_A, 2, O>;
impl<'a, const O: u8> UPSAMPLE_FOR_CVBS_W<'a, O> {
    #[doc = "27 MHz"]
    #[inline(always)]
    pub fn f27m(self) -> &'a mut W {
        self.variant(UPSAMPLE_FOR_CVBS_A::F27M)
    }
    #[doc = "54 MHz"]
    #[inline(always)]
    pub fn f54m(self) -> &'a mut W {
        self.variant(UPSAMPLE_FOR_CVBS_A::F54M)
    }
    #[doc = "108 MHz"]
    #[inline(always)]
    pub fn f108m(self) -> &'a mut W {
        self.variant(UPSAMPLE_FOR_CVBS_A::F108M)
    }
    #[doc = "216 MHz"]
    #[inline(always)]
    pub fn f216m(self) -> &'a mut W {
        self.variant(UPSAMPLE_FOR_CVBS_A::F216M)
    }
}
#[doc = "Field `upsample_for_ypbpr` reader - "]
pub type UPSAMPLE_FOR_YPBPR_R = crate::BitReader<UPSAMPLE_FOR_YPBPR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPSAMPLE_FOR_YPBPR_A {
    #[doc = "0: 1x"]
    X1 = 0,
    #[doc = "1: 2x"]
    X2 = 1,
}
impl From<UPSAMPLE_FOR_YPBPR_A> for bool {
    #[inline(always)]
    fn from(variant: UPSAMPLE_FOR_YPBPR_A) -> Self {
        variant as u8 != 0
    }
}
impl UPSAMPLE_FOR_YPBPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPSAMPLE_FOR_YPBPR_A {
        match self.bits {
            false => UPSAMPLE_FOR_YPBPR_A::X1,
            true => UPSAMPLE_FOR_YPBPR_A::X2,
        }
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == UPSAMPLE_FOR_YPBPR_A::X1
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == UPSAMPLE_FOR_YPBPR_A::X2
    }
}
#[doc = "Field `upsample_for_ypbpr` writer - "]
pub type UPSAMPLE_FOR_YPBPR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_CLOCK_GATING_SPEC, UPSAMPLE_FOR_YPBPR_A, O>;
impl<'a, const O: u8> UPSAMPLE_FOR_YPBPR_W<'a, O> {
    #[doc = "1x"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut W {
        self.variant(UPSAMPLE_FOR_YPBPR_A::X1)
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut W {
        self.variant(UPSAMPLE_FOR_YPBPR_A::X2)
    }
}
#[doc = "Field `bist_en` reader - "]
pub type BIST_EN_R = crate::BitReader<BIST_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIST_EN_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: Bist mode"]
    B_IST = 1,
}
impl From<BIST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BIST_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BIST_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIST_EN_A {
        match self.bits {
            false => BIST_EN_A::NORMAL,
            true => BIST_EN_A::B_IST,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BIST_EN_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `B_IST`"]
    #[inline(always)]
    pub fn is_b_ist(&self) -> bool {
        *self == BIST_EN_A::B_IST
    }
}
#[doc = "Field `bist_en` writer - "]
pub type BIST_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_CLOCK_GATING_SPEC, BIST_EN_A, O>;
impl<'a, const O: u8> BIST_EN_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(BIST_EN_A::NORMAL)
    }
    #[doc = "Bist mode"]
    #[inline(always)]
    pub fn b_ist(self) -> &'a mut W {
        self.variant(BIST_EN_A::B_IST)
    }
}
#[doc = "Field `clock_gate_dis` reader - "]
pub type CLOCK_GATE_DIS_R = crate::BitReader<CLOCK_GATE_DIS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLOCK_GATE_DIS_A {
    #[doc = "0: Enable"]
    ENABLE = 0,
    #[doc = "1: Disable"]
    DISABLE = 1,
}
impl From<CLOCK_GATE_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: CLOCK_GATE_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl CLOCK_GATE_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLOCK_GATE_DIS_A {
        match self.bits {
            false => CLOCK_GATE_DIS_A::ENABLE,
            true => CLOCK_GATE_DIS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLOCK_GATE_DIS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CLOCK_GATE_DIS_A::DISABLE
    }
}
#[doc = "Field `clock_gate_dis` writer - "]
pub type CLOCK_GATE_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_CLOCK_GATING_SPEC, CLOCK_GATE_DIS_A, O>;
impl<'a, const O: u8> CLOCK_GATE_DIS_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLOCK_GATE_DIS_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CLOCK_GATE_DIS_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Video Encoder enable, default disable, write 1 to take it out of the reset state"]
    #[inline(always)]
    pub fn tve_en(&self) -> TVE_EN_R {
        TVE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn upsample_for_cvbs(&self) -> UPSAMPLE_FOR_CVBS_R {
        UPSAMPLE_FOR_CVBS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn upsample_for_ypbpr(&self) -> UPSAMPLE_FOR_YPBPR_R {
        UPSAMPLE_FOR_YPBPR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn bist_en(&self) -> BIST_EN_R {
        BIST_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clock_gate_dis(&self) -> CLOCK_GATE_DIS_R {
        CLOCK_GATE_DIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Video Encoder enable, default disable, write 1 to take it out of the reset state"]
    #[inline(always)]
    #[must_use]
    pub fn tve_en(&mut self) -> TVE_EN_W<0> {
        TVE_EN_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn upsample_for_cvbs(&mut self) -> UPSAMPLE_FOR_CVBS_W<20> {
        UPSAMPLE_FOR_CVBS_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn upsample_for_ypbpr(&mut self) -> UPSAMPLE_FOR_YPBPR_W<22> {
        UPSAMPLE_FOR_YPBPR_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn bist_en(&mut self) -> BIST_EN_W<28> {
        BIST_EN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn clock_gate_dis(&mut self) -> CLOCK_GATE_DIS_W<31> {
        CLOCK_GATE_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Clock Gating Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_clock_gating](index.html) module"]
pub struct TVE_CLOCK_GATING_SPEC;
impl crate::RegisterSpec for TVE_CLOCK_GATING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_clock_gating::R](R) reader structure"]
impl crate::Readable for TVE_CLOCK_GATING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_clock_gating::W](W) writer structure"]
impl crate::Writable for TVE_CLOCK_GATING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_clock_gating to value 0"]
impl crate::Resettable for TVE_CLOCK_GATING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
