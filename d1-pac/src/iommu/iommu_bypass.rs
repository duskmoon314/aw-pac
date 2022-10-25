#[doc = "Register `iommu_bypass` reader"]
pub struct R(crate::R<IOMMU_BYPASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_BYPASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_BYPASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_BYPASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_bypass` writer"]
pub struct W(crate::W<IOMMU_BYPASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_BYPASS_SPEC>;
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
impl From<crate::W<IOMMU_BYPASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_BYPASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `m_bp[0-6]` reader - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
pub type M_BP_R = crate::BitReader<M_BP_A>;
#[doc = "Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_BP_A {
    #[doc = "0: Disable bypass function"]
    DISABLE = 0,
    #[doc = "1: Enable bypass function"]
    ENABLE = 1,
}
impl From<M_BP_A> for bool {
    #[inline(always)]
    fn from(variant: M_BP_A) -> Self {
        variant as u8 != 0
    }
}
impl M_BP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_BP_A {
        match self.bits {
            false => M_BP_A::DISABLE,
            true => M_BP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == M_BP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == M_BP_A::ENABLE
    }
}
#[doc = "Field `m_bp[0-6]` writer - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
pub type M_BP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOMMU_BYPASS_SPEC, M_BP_A, O>;
impl<'a, const O: u8> M_BP_W<'a, O> {
    #[doc = "Disable bypass function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(M_BP_A::DISABLE)
    }
    #[doc = "Enable bypass function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(M_BP_A::ENABLE)
    }
}
impl R {
    #[doc = "Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    pub unsafe fn m_bp(&self, n: u8) -> M_BP_R {
        M_BP_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    pub fn m0_bp(&self) -> M_BP_R {
        M_BP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    pub fn m1_bp(&self) -> M_BP_R {
        M_BP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    pub fn m2_bp(&self) -> M_BP_R {
        M_BP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    pub fn m3_bp(&self) -> M_BP_R {
        M_BP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    pub fn m4_bp(&self) -> M_BP_R {
        M_BP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    pub fn m5_bp(&self) -> M_BP_R {
        M_BP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    pub fn m6_bp(&self) -> M_BP_R {
        M_BP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn m_bp<const O: u8>(&mut self) -> M_BP_W<O> {
        M_BP_W::new(self)
    }
    #[doc = "Bit 0 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    #[must_use]
    pub fn m0_bp(&mut self) -> M_BP_W<0> {
        M_BP_W::new(self)
    }
    #[doc = "Bit 1 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    #[must_use]
    pub fn m1_bp(&mut self) -> M_BP_W<1> {
        M_BP_W::new(self)
    }
    #[doc = "Bit 2 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    #[must_use]
    pub fn m2_bp(&mut self) -> M_BP_W<2> {
        M_BP_W::new(self)
    }
    #[doc = "Bit 3 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    #[must_use]
    pub fn m3_bp(&mut self) -> M_BP_W<3> {
        M_BP_W::new(self)
    }
    #[doc = "Bit 4 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    #[must_use]
    pub fn m4_bp(&mut self) -> M_BP_W<4> {
        M_BP_W::new(self)
    }
    #[doc = "Bit 5 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    #[must_use]
    pub fn m5_bp(&mut self) -> M_BP_W<5> {
        M_BP_W::new(self)
    }
    #[doc = "Bit 6 - Master\\[i\\] bypass switch\n\nAfter bypass function is opened, IOMMU can not map the address of Master6 sending, and directly output the virtual address to MBUS as physical address."]
    #[inline(always)]
    #[must_use]
    pub fn m6_bp(&mut self) -> M_BP_W<6> {
        M_BP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU Bypass Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_bypass](index.html) module"]
pub struct IOMMU_BYPASS_SPEC;
impl crate::RegisterSpec for IOMMU_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_bypass::R](R) reader structure"]
impl crate::Readable for IOMMU_BYPASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_bypass::W](W) writer structure"]
impl crate::Writable for IOMMU_BYPASS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_bypass to value 0x7f"]
impl crate::Resettable for IOMMU_BYPASS_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
