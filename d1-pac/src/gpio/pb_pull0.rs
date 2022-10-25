#[doc = "Register `pb_pull0` reader"]
pub struct R(crate::R<PB_PULL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB_PULL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB_PULL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB_PULL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pb_pull0` writer"]
pub struct W(crate::W<PB_PULL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB_PULL0_SPEC>;
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
impl From<crate::W<PB_PULL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB_PULL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pc_pull[0-12]` reader - PC Pull_up/down Select"]
pub type PC_PULL_R = crate::FieldReader<u8, PC_PULL_A>;
#[doc = "PC Pull_up/down Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PC_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PC_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PC_PULL_A) -> Self {
        variant as _
    }
}
impl PC_PULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PC_PULL_A {
        match self.bits {
            0 => PC_PULL_A::PULL_DISABLE,
            1 => PC_PULL_A::PULL_UP,
            2 => PC_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        *self == PC_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PC_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PC_PULL_A::PULL_DOWN
    }
}
#[doc = "Field `pc_pull[0-12]` writer - PC Pull_up/down Select"]
pub type PC_PULL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PB_PULL0_SPEC, u8, PC_PULL_A, 2, O>;
impl<'a, const O: u8> PC_PULL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PC_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PC_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PC_PULL_A::PULL_DOWN)
    }
}
impl R {
    #[doc = "PC Pull_up/down Select"]
    #[inline(always)]
    pub unsafe fn pc_pull(&self, n: u8) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc0_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc1_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc2_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc3_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc4_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc5_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc6_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc7_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc8_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc9_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc10_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc11_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc12_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn pc_pull<const O: u8>(&mut self) -> PC_PULL_W<O> {
        PC_PULL_W::new(self)
    }
    #[doc = "Bits 0:1 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc0_pull(&mut self) -> PC_PULL_W<0> {
        PC_PULL_W::new(self)
    }
    #[doc = "Bits 2:3 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc1_pull(&mut self) -> PC_PULL_W<2> {
        PC_PULL_W::new(self)
    }
    #[doc = "Bits 4:5 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc2_pull(&mut self) -> PC_PULL_W<4> {
        PC_PULL_W::new(self)
    }
    #[doc = "Bits 6:7 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc3_pull(&mut self) -> PC_PULL_W<6> {
        PC_PULL_W::new(self)
    }
    #[doc = "Bits 8:9 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc4_pull(&mut self) -> PC_PULL_W<8> {
        PC_PULL_W::new(self)
    }
    #[doc = "Bits 10:11 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc5_pull(&mut self) -> PC_PULL_W<10> {
        PC_PULL_W::new(self)
    }
    #[doc = "Bits 12:13 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc6_pull(&mut self) -> PC_PULL_W<12> {
        PC_PULL_W::new(self)
    }
    #[doc = "Bits 14:15 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc7_pull(&mut self) -> PC_PULL_W<14> {
        PC_PULL_W::new(self)
    }
    #[doc = "Bits 16:17 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc8_pull(&mut self) -> PC_PULL_W<16> {
        PC_PULL_W::new(self)
    }
    #[doc = "Bits 18:19 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc9_pull(&mut self) -> PC_PULL_W<18> {
        PC_PULL_W::new(self)
    }
    #[doc = "Bits 20:21 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc10_pull(&mut self) -> PC_PULL_W<20> {
        PC_PULL_W::new(self)
    }
    #[doc = "Bits 22:23 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc11_pull(&mut self) -> PC_PULL_W<22> {
        PC_PULL_W::new(self)
    }
    #[doc = "Bits 24:25 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc12_pull(&mut self) -> PC_PULL_W<24> {
        PC_PULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PB Pull Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_pull0](index.html) module"]
pub struct PB_PULL0_SPEC;
impl crate::RegisterSpec for PB_PULL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb_pull0::R](R) reader structure"]
impl crate::Readable for PB_PULL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb_pull0::W](W) writer structure"]
impl crate::Writable for PB_PULL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pb_pull0 to value 0"]
impl crate::Resettable for PB_PULL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
