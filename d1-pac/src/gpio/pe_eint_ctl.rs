#[doc = "Register `pe_eint_ctl` reader"]
pub struct R(crate::R<PE_EINT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE_EINT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PE_EINT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PE_EINT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pe_eint_ctl` writer"]
pub struct W(crate::W<PE_EINT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE_EINT_CTL_SPEC>;
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
impl From<crate::W<PE_EINT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PE_EINT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External INT Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Fields `EINT(0-17)_CTL` reader - External INT Enable"]
pub struct EINT_CTL_R(crate::FieldReader<bool, EINT_CTL_A>);
impl EINT_CTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EINT_CTL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == EINT_CTL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == EINT_CTL_A::ENABLE
    }
}
impl core::ops::Deref for EINT_CTL_R {
    type Target = crate::FieldReader<bool, EINT_CTL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `EINT(0-17)_CTL` writer - External INT Enable"]
pub struct EINT_CTL_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> EINT_CTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EINT_CTL_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << self.offset)) | ((value as u32 & 1) << self.offset);
        self.w
    }
}
#[doc = "Fields `EINT(0-17)_CTL` const generic writer - External INT Enable"]
pub struct EINT_CTL_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> EINT_CTL_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EINT_CTL_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << O)) | ((value as u32 & 1) << O);
        self.w
    }
}
impl R {
    #[doc = "External INT Enable"]
    #[inline(always)]
    pub unsafe fn eint_ctl(&self, n: usize) -> EINT_CTL_R {
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
    #[doc = "Bit 7 - External INT Enable"]
    #[inline(always)]
    pub fn eint7_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External INT Enable"]
    #[inline(always)]
    pub fn eint8_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External INT Enable"]
    #[inline(always)]
    pub fn eint9_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External INT Enable"]
    #[inline(always)]
    pub fn eint10_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External INT Enable"]
    #[inline(always)]
    pub fn eint11_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External INT Enable"]
    #[inline(always)]
    pub fn eint12_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External INT Enable"]
    #[inline(always)]
    pub fn eint13_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External INT Enable"]
    #[inline(always)]
    pub fn eint14_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External INT Enable"]
    #[inline(always)]
    pub fn eint15_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - External INT Enable"]
    #[inline(always)]
    pub fn eint16_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External INT Enable"]
    #[inline(always)]
    pub fn eint17_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "External INT Enable"]
    #[inline(always)]
    pub unsafe fn eint_ctl(&mut self, n: usize) -> EINT_CTL_W {
        EINT_CTL_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - External INT Enable"]
    #[inline(always)]
    pub fn eint0_ctl(&mut self) -> EINT_CTL_CGW<0> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 1 - External INT Enable"]
    #[inline(always)]
    pub fn eint1_ctl(&mut self) -> EINT_CTL_CGW<1> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 2 - External INT Enable"]
    #[inline(always)]
    pub fn eint2_ctl(&mut self) -> EINT_CTL_CGW<2> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 3 - External INT Enable"]
    #[inline(always)]
    pub fn eint3_ctl(&mut self) -> EINT_CTL_CGW<3> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 4 - External INT Enable"]
    #[inline(always)]
    pub fn eint4_ctl(&mut self) -> EINT_CTL_CGW<4> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 5 - External INT Enable"]
    #[inline(always)]
    pub fn eint5_ctl(&mut self) -> EINT_CTL_CGW<5> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 6 - External INT Enable"]
    #[inline(always)]
    pub fn eint6_ctl(&mut self) -> EINT_CTL_CGW<6> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 7 - External INT Enable"]
    #[inline(always)]
    pub fn eint7_ctl(&mut self) -> EINT_CTL_CGW<7> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 8 - External INT Enable"]
    #[inline(always)]
    pub fn eint8_ctl(&mut self) -> EINT_CTL_CGW<8> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 9 - External INT Enable"]
    #[inline(always)]
    pub fn eint9_ctl(&mut self) -> EINT_CTL_CGW<9> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 10 - External INT Enable"]
    #[inline(always)]
    pub fn eint10_ctl(&mut self) -> EINT_CTL_CGW<10> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 11 - External INT Enable"]
    #[inline(always)]
    pub fn eint11_ctl(&mut self) -> EINT_CTL_CGW<11> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 12 - External INT Enable"]
    #[inline(always)]
    pub fn eint12_ctl(&mut self) -> EINT_CTL_CGW<12> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 13 - External INT Enable"]
    #[inline(always)]
    pub fn eint13_ctl(&mut self) -> EINT_CTL_CGW<13> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 14 - External INT Enable"]
    #[inline(always)]
    pub fn eint14_ctl(&mut self) -> EINT_CTL_CGW<14> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 15 - External INT Enable"]
    #[inline(always)]
    pub fn eint15_ctl(&mut self) -> EINT_CTL_CGW<15> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 16 - External INT Enable"]
    #[inline(always)]
    pub fn eint16_ctl(&mut self) -> EINT_CTL_CGW<16> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Bit 17 - External INT Enable"]
    #[inline(always)]
    pub fn eint17_ctl(&mut self) -> EINT_CTL_CGW<17> {
        EINT_CTL_CGW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PE External Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_eint_ctl](index.html) module"]
pub struct PE_EINT_CTL_SPEC;
impl crate::RegisterSpec for PE_EINT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pe_eint_ctl::R](R) reader structure"]
impl crate::Readable for PE_EINT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe_eint_ctl::W](W) writer structure"]
impl crate::Writable for PE_EINT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pe_eint_ctl to value 0"]
impl crate::Resettable for PE_EINT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
