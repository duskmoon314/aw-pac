#[doc = "Register `ctrl` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ctrl` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctrl` reader - PLIC Control"]
pub type CTRL_R = crate::BitReader<CTRL_A>;
#[doc = "PLIC Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTRL_A {
    #[doc = "0: Only the machine mode can access to all registers in PLIC. Supervisor mode can only access the interrupt threshold register and the interrupt response/completion register."]
    M = 0,
    #[doc = "1: The machine mode and the supervisor mode can access all registers. CTRL is accessible only in the machine mode."]
    MS = 1,
}
impl From<CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRL_A {
        match self.bits {
            false => CTRL_A::M,
            true => CTRL_A::MS,
        }
    }
    #[doc = "Checks if the value of the field is `M`"]
    #[inline(always)]
    pub fn is_m(&self) -> bool {
        *self == CTRL_A::M
    }
    #[doc = "Checks if the value of the field is `MS`"]
    #[inline(always)]
    pub fn is_ms(&self) -> bool {
        *self == CTRL_A::MS
    }
}
#[doc = "Field `ctrl` writer - PLIC Control"]
pub type CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, CTRL_A, O>;
impl<'a, const O: u8> CTRL_W<'a, O> {
    #[doc = "Only the machine mode can access to all registers in PLIC. Supervisor mode can only access the interrupt threshold register and the interrupt response/completion register."]
    #[inline(always)]
    pub fn m(self) -> &'a mut W {
        self.variant(CTRL_A::M)
    }
    #[doc = "The machine mode and the supervisor mode can access all registers. CTRL is accessible only in the machine mode."]
    #[inline(always)]
    pub fn ms(self) -> &'a mut W {
        self.variant(CTRL_A::MS)
    }
}
impl R {
    #[doc = "Bit 0 - PLIC Control"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLIC Control"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl(&mut self) -> CTRL_W<0> {
        CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ctrl to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
