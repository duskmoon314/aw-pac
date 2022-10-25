#[doc = "Register `iommu_ooo_ctrl` reader"]
pub struct R(crate::R<IOMMU_OOO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_OOO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_OOO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_OOO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_ooo_ctrl` writer"]
pub struct W(crate::W<IOMMU_OOO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_OOO_CTRL_SPEC>;
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
impl From<crate::W<IOMMU_OOO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_OOO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `m_ooo_ctrl[0-6]` reader - Master\\[i\\] out-of-order control bit"]
pub type M_OOO_CTRL_R = crate::BitReader<M_OOO_CTRL_A>;
#[doc = "Master\\[i\\] out-of-order control bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_OOO_CTRL_A {
    #[doc = "0: Disable out-of-order"]
    DISABLE = 0,
    #[doc = "1: Enable out-of-order"]
    ENABLE = 1,
}
impl From<M_OOO_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: M_OOO_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl M_OOO_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_OOO_CTRL_A {
        match self.bits {
            false => M_OOO_CTRL_A::DISABLE,
            true => M_OOO_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == M_OOO_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == M_OOO_CTRL_A::ENABLE
    }
}
#[doc = "Field `m_ooo_ctrl[0-6]` writer - Master\\[i\\] out-of-order control bit"]
pub type M_OOO_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_OOO_CTRL_SPEC, M_OOO_CTRL_A, O>;
impl<'a, const O: u8> M_OOO_CTRL_W<'a, O> {
    #[doc = "Disable out-of-order"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(M_OOO_CTRL_A::DISABLE)
    }
    #[doc = "Enable out-of-order"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(M_OOO_CTRL_A::ENABLE)
    }
}
impl R {
    #[doc = "Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    pub unsafe fn m_ooo_ctrl(&self, n: u8) -> M_OOO_CTRL_R {
        M_OOO_CTRL_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    pub fn m0_ooo_ctrl(&self) -> M_OOO_CTRL_R {
        M_OOO_CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    pub fn m1_ooo_ctrl(&self) -> M_OOO_CTRL_R {
        M_OOO_CTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    pub fn m2_ooo_ctrl(&self) -> M_OOO_CTRL_R {
        M_OOO_CTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    pub fn m3_ooo_ctrl(&self) -> M_OOO_CTRL_R {
        M_OOO_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    pub fn m4_ooo_ctrl(&self) -> M_OOO_CTRL_R {
        M_OOO_CTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    pub fn m5_ooo_ctrl(&self) -> M_OOO_CTRL_R {
        M_OOO_CTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    pub fn m6_ooo_ctrl(&self) -> M_OOO_CTRL_R {
        M_OOO_CTRL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn m_ooo_ctrl<const O: u8>(&mut self) -> M_OOO_CTRL_W<O> {
        M_OOO_CTRL_W::new(self)
    }
    #[doc = "Bit 0 - Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    #[must_use]
    pub fn m0_ooo_ctrl(&mut self) -> M_OOO_CTRL_W<0> {
        M_OOO_CTRL_W::new(self)
    }
    #[doc = "Bit 1 - Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    #[must_use]
    pub fn m1_ooo_ctrl(&mut self) -> M_OOO_CTRL_W<1> {
        M_OOO_CTRL_W::new(self)
    }
    #[doc = "Bit 2 - Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    #[must_use]
    pub fn m2_ooo_ctrl(&mut self) -> M_OOO_CTRL_W<2> {
        M_OOO_CTRL_W::new(self)
    }
    #[doc = "Bit 3 - Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    #[must_use]
    pub fn m3_ooo_ctrl(&mut self) -> M_OOO_CTRL_W<3> {
        M_OOO_CTRL_W::new(self)
    }
    #[doc = "Bit 4 - Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    #[must_use]
    pub fn m4_ooo_ctrl(&mut self) -> M_OOO_CTRL_W<4> {
        M_OOO_CTRL_W::new(self)
    }
    #[doc = "Bit 5 - Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    #[must_use]
    pub fn m5_ooo_ctrl(&mut self) -> M_OOO_CTRL_W<5> {
        M_OOO_CTRL_W::new(self)
    }
    #[doc = "Bit 6 - Master\\[i\\] out-of-order control bit"]
    #[inline(always)]
    #[must_use]
    pub fn m6_ooo_ctrl(&mut self) -> M_OOO_CTRL_W<6> {
        M_OOO_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU Out of Order Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_ooo_ctrl](index.html) module"]
pub struct IOMMU_OOO_CTRL_SPEC;
impl crate::RegisterSpec for IOMMU_OOO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_ooo_ctrl::R](R) reader structure"]
impl crate::Readable for IOMMU_OOO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_ooo_ctrl::W](W) writer structure"]
impl crate::Writable for IOMMU_OOO_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_ooo_ctrl to value 0x7f"]
impl crate::Resettable for IOMMU_OOO_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
