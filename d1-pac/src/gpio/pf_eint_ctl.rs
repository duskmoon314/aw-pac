#[doc = "Register `pf_eint_ctl` reader"]
pub struct R(crate::R<PF_EINT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_EINT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_EINT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_EINT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pf_eint_ctl` writer"]
pub struct W(crate::W<PF_EINT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_EINT_CTL_SPEC>;
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
impl From<crate::W<PF_EINT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF_EINT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `eint_ctl[0-6]` reader - External INT Enable"]
pub type EINT_CTL_R = crate::BitReader<EINT_CTL_A>;
#[doc = "External INT Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EINT_CTL_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<EINT_CTL_A> for bool {
    #[inline(always)]
    fn from(variant: EINT_CTL_A) -> Self {
        variant as u8 != 0
    }
}
impl EINT_CTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT_CTL_A {
        match self.bits {
            false => EINT_CTL_A::DISABLE,
            true => EINT_CTL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EINT_CTL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EINT_CTL_A::ENABLE
    }
}
#[doc = "Field `eint_ctl[0-6]` writer - External INT Enable"]
pub type EINT_CTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PF_EINT_CTL_SPEC, EINT_CTL_A, O>;
impl<'a, const O: u8> EINT_CTL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EINT_CTL_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EINT_CTL_A::ENABLE)
    }
}
impl R {
    #[doc = "External INT Enable"]
    #[inline(always)]
    pub unsafe fn eint_ctl(&self, n: u8) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - External INT Enable"]
    #[inline(always)]
    pub fn eint0_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External INT Enable"]
    #[inline(always)]
    pub fn eint1_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External INT Enable"]
    #[inline(always)]
    pub fn eint2_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External INT Enable"]
    #[inline(always)]
    pub fn eint3_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External INT Enable"]
    #[inline(always)]
    pub fn eint4_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External INT Enable"]
    #[inline(always)]
    pub fn eint5_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External INT Enable"]
    #[inline(always)]
    pub fn eint6_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn eint_ctl<const O: u8>(&mut self) -> EINT_CTL_W<O> {
        EINT_CTL_W::new(self)
    }
    #[doc = "Bit 0 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint0_ctl(&mut self) -> EINT_CTL_W<0> {
        EINT_CTL_W::new(self)
    }
    #[doc = "Bit 1 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint1_ctl(&mut self) -> EINT_CTL_W<1> {
        EINT_CTL_W::new(self)
    }
    #[doc = "Bit 2 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint2_ctl(&mut self) -> EINT_CTL_W<2> {
        EINT_CTL_W::new(self)
    }
    #[doc = "Bit 3 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint3_ctl(&mut self) -> EINT_CTL_W<3> {
        EINT_CTL_W::new(self)
    }
    #[doc = "Bit 4 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint4_ctl(&mut self) -> EINT_CTL_W<4> {
        EINT_CTL_W::new(self)
    }
    #[doc = "Bit 5 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint5_ctl(&mut self) -> EINT_CTL_W<5> {
        EINT_CTL_W::new(self)
    }
    #[doc = "Bit 6 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint6_ctl(&mut self) -> EINT_CTL_W<6> {
        EINT_CTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PF External Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_eint_ctl](index.html) module"]
pub struct PF_EINT_CTL_SPEC;
impl crate::RegisterSpec for PF_EINT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_eint_ctl::R](R) reader structure"]
impl crate::Readable for PF_EINT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_eint_ctl::W](W) writer structure"]
impl crate::Writable for PF_EINT_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pf_eint_ctl to value 0"]
impl crate::Resettable for PF_EINT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
