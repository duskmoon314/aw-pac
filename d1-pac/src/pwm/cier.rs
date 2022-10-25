#[doc = "Register `cier` reader"]
pub struct R(crate::R<CIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cier` writer"]
pub struct W(crate::W<CIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIER_SPEC>;
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
impl From<crate::W<CIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `crie[0-7]` reader - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
pub type CRIE_R = crate::BitReader<CRIE_A>;
#[doc = "If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRIE_A {
    #[doc = "0: Capture channel rise lock interrupt disable"]
    DISABLE = 0,
    #[doc = "1: Capture channel rise lock interrupt enable"]
    ENABLE = 1,
}
impl From<CRIE_A> for bool {
    #[inline(always)]
    fn from(variant: CRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRIE_A {
        match self.bits {
            false => CRIE_A::DISABLE,
            true => CRIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CRIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CRIE_A::ENABLE
    }
}
#[doc = "Field `crie[0-7]` writer - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
pub type CRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, CRIE_A, O>;
impl<'a, const O: u8> CRIE_W<'a, O> {
    #[doc = "Capture channel rise lock interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRIE_A::DISABLE)
    }
    #[doc = "Capture channel rise lock interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CRIE_A::ENABLE)
    }
}
#[doc = "Field `cfie[0-7]` reader - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
pub type CFIE_R = crate::BitReader<CFIE_A>;
#[doc = "If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFIE_A {
    #[doc = "0: Capture channel fall lock interrupt disable"]
    DISABLE = 0,
    #[doc = "1: Capture channel fall lock interrupt enable"]
    ENABLE = 1,
}
impl From<CFIE_A> for bool {
    #[inline(always)]
    fn from(variant: CFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFIE_A {
        match self.bits {
            false => CFIE_A::DISABLE,
            true => CFIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CFIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CFIE_A::ENABLE
    }
}
#[doc = "Field `cfie[0-7]` writer - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
pub type CFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, CFIE_A, O>;
impl<'a, const O: u8> CFIE_W<'a, O> {
    #[doc = "Capture channel fall lock interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CFIE_A::DISABLE)
    }
    #[doc = "Capture channel fall lock interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CFIE_A::ENABLE)
    }
}
impl R {
    #[doc = "If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub unsafe fn crie(&self, n: u8) -> CRIE_R {
        CRIE_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    #[doc = "Bit 0 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie0(&self) -> CRIE_R {
        CRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie1(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie2(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie3(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie4(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie5(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie6(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie7(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub unsafe fn cfie(&self, n: u8) -> CFIE_R {
        CFIE_R::new(((self.bits >> (n * 2 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie0(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie1(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie2(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie3(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie4(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie5(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie6(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie7(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn crie<const O: u8>(&mut self) -> CRIE_W<O> {
        CRIE_W::new(self)
    }
    #[doc = "Bit 0 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie0(&mut self) -> CRIE_W<0> {
        CRIE_W::new(self)
    }
    #[doc = "Bit 2 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie1(&mut self) -> CRIE_W<2> {
        CRIE_W::new(self)
    }
    #[doc = "Bit 4 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie2(&mut self) -> CRIE_W<4> {
        CRIE_W::new(self)
    }
    #[doc = "Bit 6 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie3(&mut self) -> CRIE_W<6> {
        CRIE_W::new(self)
    }
    #[doc = "Bit 8 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie4(&mut self) -> CRIE_W<8> {
        CRIE_W::new(self)
    }
    #[doc = "Bit 10 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie5(&mut self) -> CRIE_W<10> {
        CRIE_W::new(self)
    }
    #[doc = "Bit 12 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie6(&mut self) -> CRIE_W<12> {
        CRIE_W::new(self)
    }
    #[doc = "Bit 14 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie7(&mut self) -> CRIE_W<14> {
        CRIE_W::new(self)
    }
    #[doc = "If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn cfie<const O: u8>(&mut self) -> CFIE_W<O> {
        CFIE_W::new(self)
    }
    #[doc = "Bit 1 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie0(&mut self) -> CFIE_W<1> {
        CFIE_W::new(self)
    }
    #[doc = "Bit 3 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie1(&mut self) -> CFIE_W<3> {
        CFIE_W::new(self)
    }
    #[doc = "Bit 5 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie2(&mut self) -> CFIE_W<5> {
        CFIE_W::new(self)
    }
    #[doc = "Bit 7 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie3(&mut self) -> CFIE_W<7> {
        CFIE_W::new(self)
    }
    #[doc = "Bit 9 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie4(&mut self) -> CFIE_W<9> {
        CFIE_W::new(self)
    }
    #[doc = "Bit 11 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie5(&mut self) -> CFIE_W<11> {
        CFIE_W::new(self)
    }
    #[doc = "Bit 13 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie6(&mut self) -> CFIE_W<13> {
        CFIE_W::new(self)
    }
    #[doc = "Bit 15 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie7(&mut self) -> CFIE_W<15> {
        CFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture IRQ Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cier](index.html) module"]
pub struct CIER_SPEC;
impl crate::RegisterSpec for CIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cier::R](R) reader structure"]
impl crate::Readable for CIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cier::W](W) writer structure"]
impl crate::Writable for CIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cier to value 0"]
impl crate::Resettable for CIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
