#[doc = "Register `iommu_reset` reader"]
pub struct R(crate::R<IOMMU_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `iommu_reset` writer"]
pub struct W(crate::W<IOMMU_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMMU_RESET_SPEC>;
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
impl From<crate::W<IOMMU_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMMU_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `m_rst[0-6]` reader - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
pub type M_RST_R = crate::BitReader<M_RST_A>;
#[doc = "Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_RST_A {
    #[doc = "0: Set reset signal"]
    SET = 0,
    #[doc = "1: Release reset signal"]
    R_ELEASE = 1,
}
impl From<M_RST_A> for bool {
    #[inline(always)]
    fn from(variant: M_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl M_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_RST_A {
        match self.bits {
            false => M_RST_A::SET,
            true => M_RST_A::R_ELEASE,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == M_RST_A::SET
    }
    #[doc = "Checks if the value of the field is `R_ELEASE`"]
    #[inline(always)]
    pub fn is_r_elease(&self) -> bool {
        *self == M_RST_A::R_ELEASE
    }
}
#[doc = "Field `m_rst[0-6]` writer - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
pub type M_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOMMU_RESET_SPEC, M_RST_A, O>;
impl<'a, const O: u8> M_RST_W<'a, O> {
    #[doc = "Set reset signal"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(M_RST_A::SET)
    }
    #[doc = "Release reset signal"]
    #[inline(always)]
    pub fn r_elease(self) -> &'a mut W {
        self.variant(M_RST_A::R_ELEASE)
    }
}
#[doc = "Field `mtlb_rst` reader - Macrotlb Reset\n\nMacro TLB address convert lane software reset switch.\n\nWhen PTW Cache occurs abnormal, the bit is used to reset PTW Cache individually."]
pub type MTLB_RST_R = crate::BitReader<MTLB_RST_A>;
#[doc = "Macrotlb Reset\n\nMacro TLB address convert lane software reset switch.\n\nWhen PTW Cache occurs abnormal, the bit is used to reset PTW Cache individually.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTLB_RST_A {
    #[doc = "0: Set reset signal"]
    SET = 0,
    #[doc = "1: Release reset signal"]
    R_ELEASE = 1,
}
impl From<MTLB_RST_A> for bool {
    #[inline(always)]
    fn from(variant: MTLB_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl MTLB_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTLB_RST_A {
        match self.bits {
            false => MTLB_RST_A::SET,
            true => MTLB_RST_A::R_ELEASE,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == MTLB_RST_A::SET
    }
    #[doc = "Checks if the value of the field is `R_ELEASE`"]
    #[inline(always)]
    pub fn is_r_elease(&self) -> bool {
        *self == MTLB_RST_A::R_ELEASE
    }
}
#[doc = "Field `mtlb_rst` writer - Macrotlb Reset\n\nMacro TLB address convert lane software reset switch.\n\nWhen PTW Cache occurs abnormal, the bit is used to reset PTW Cache individually."]
pub type MTLB_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOMMU_RESET_SPEC, MTLB_RST_A, O>;
impl<'a, const O: u8> MTLB_RST_W<'a, O> {
    #[doc = "Set reset signal"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(MTLB_RST_A::SET)
    }
    #[doc = "Release reset signal"]
    #[inline(always)]
    pub fn r_elease(self) -> &'a mut W {
        self.variant(MTLB_RST_A::R_ELEASE)
    }
}
#[doc = "Field `pc_rst` reader - PTW Cache Reset\n\nPTW Cache address convert lane software reset switch.\n\nWhen PTW Cache occurs abnormal, the bit is used to reset PTW Cache individually."]
pub type PC_RST_R = crate::BitReader<PC_RST_A>;
#[doc = "PTW Cache Reset\n\nPTW Cache address convert lane software reset switch.\n\nWhen PTW Cache occurs abnormal, the bit is used to reset PTW Cache individually.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PC_RST_A {
    #[doc = "0: Set reset signal"]
    SET = 0,
    #[doc = "1: Release reset signal"]
    R_ELEASE = 1,
}
impl From<PC_RST_A> for bool {
    #[inline(always)]
    fn from(variant: PC_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl PC_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PC_RST_A {
        match self.bits {
            false => PC_RST_A::SET,
            true => PC_RST_A::R_ELEASE,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == PC_RST_A::SET
    }
    #[doc = "Checks if the value of the field is `R_ELEASE`"]
    #[inline(always)]
    pub fn is_r_elease(&self) -> bool {
        *self == PC_RST_A::R_ELEASE
    }
}
#[doc = "Field `pc_rst` writer - PTW Cache Reset\n\nPTW Cache address convert lane software reset switch.\n\nWhen PTW Cache occurs abnormal, the bit is used to reset PTW Cache individually."]
pub type PC_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOMMU_RESET_SPEC, PC_RST_A, O>;
impl<'a, const O: u8> PC_RST_W<'a, O> {
    #[doc = "Set reset signal"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PC_RST_A::SET)
    }
    #[doc = "Release reset signal"]
    #[inline(always)]
    pub fn r_elease(self) -> &'a mut W {
        self.variant(PC_RST_A::R_ELEASE)
    }
}
#[doc = "Field `iommu_reset` reader - IOMMU Software Reset Switch、n\nBefore IOMMU software reset operation, ensure IOMMU never be opened; or all bus operations are completed; or DRAM and the peripherals have opened the corresponding switch, for shielding the effects of IOMMU reset."]
pub type IOMMU_RESET_R = crate::BitReader<IOMMU_RESET_A>;
#[doc = "IOMMU Software Reset Switch、n\nBefore IOMMU software reset operation, ensure IOMMU never be opened; or all bus operations are completed; or DRAM and the peripherals have opened the corresponding switch, for shielding the effects of IOMMU reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMMU_RESET_A {
    #[doc = "0: Set reset signal"]
    SET = 0,
    #[doc = "1: Release reset signal"]
    R_ELEASE = 1,
}
impl From<IOMMU_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: IOMMU_RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMMU_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMMU_RESET_A {
        match self.bits {
            false => IOMMU_RESET_A::SET,
            true => IOMMU_RESET_A::R_ELEASE,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IOMMU_RESET_A::SET
    }
    #[doc = "Checks if the value of the field is `R_ELEASE`"]
    #[inline(always)]
    pub fn is_r_elease(&self) -> bool {
        *self == IOMMU_RESET_A::R_ELEASE
    }
}
#[doc = "Field `iommu_reset` writer - IOMMU Software Reset Switch、n\nBefore IOMMU software reset operation, ensure IOMMU never be opened; or all bus operations are completed; or DRAM and the peripherals have opened the corresponding switch, for shielding the effects of IOMMU reset."]
pub type IOMMU_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IOMMU_RESET_SPEC, IOMMU_RESET_A, O>;
impl<'a, const O: u8> IOMMU_RESET_W<'a, O> {
    #[doc = "Set reset signal"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(IOMMU_RESET_A::SET)
    }
    #[doc = "Release reset signal"]
    #[inline(always)]
    pub fn r_elease(self) -> &'a mut W {
        self.variant(IOMMU_RESET_A::R_ELEASE)
    }
}
impl R {
    #[doc = "Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    pub unsafe fn m_rst(&self, n: u8) -> M_RST_R {
        M_RST_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    pub fn m0_rst(&self) -> M_RST_R {
        M_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    pub fn m1_rst(&self) -> M_RST_R {
        M_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    pub fn m2_rst(&self) -> M_RST_R {
        M_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    pub fn m3_rst(&self) -> M_RST_R {
        M_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    pub fn m4_rst(&self) -> M_RST_R {
        M_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    pub fn m5_rst(&self) -> M_RST_R {
        M_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    pub fn m6_rst(&self) -> M_RST_R {
        M_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - Macrotlb Reset\n\nMacro TLB address convert lane software reset switch.\n\nWhen PTW Cache occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    pub fn mtlb_rst(&self) -> MTLB_RST_R {
        MTLB_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PTW Cache Reset\n\nPTW Cache address convert lane software reset switch.\n\nWhen PTW Cache occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    pub fn pc_rst(&self) -> PC_RST_R {
        PC_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - IOMMU Software Reset Switch、n\nBefore IOMMU software reset operation, ensure IOMMU never be opened; or all bus operations are completed; or DRAM and the peripherals have opened the corresponding switch, for shielding the effects of IOMMU reset."]
    #[inline(always)]
    pub fn iommu_reset(&self) -> IOMMU_RESET_R {
        IOMMU_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn m_rst<const O: u8>(&mut self) -> M_RST_W<O> {
        M_RST_W::new(self)
    }
    #[doc = "Bit 0 - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    #[must_use]
    pub fn m0_rst(&mut self) -> M_RST_W<0> {
        M_RST_W::new(self)
    }
    #[doc = "Bit 1 - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    #[must_use]
    pub fn m1_rst(&mut self) -> M_RST_W<1> {
        M_RST_W::new(self)
    }
    #[doc = "Bit 2 - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    #[must_use]
    pub fn m2_rst(&mut self) -> M_RST_W<2> {
        M_RST_W::new(self)
    }
    #[doc = "Bit 3 - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    #[must_use]
    pub fn m3_rst(&mut self) -> M_RST_W<3> {
        M_RST_W::new(self)
    }
    #[doc = "Bit 4 - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    #[must_use]
    pub fn m4_rst(&mut self) -> M_RST_W<4> {
        M_RST_W::new(self)
    }
    #[doc = "Bit 5 - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    #[must_use]
    pub fn m5_rst(&mut self) -> M_RST_W<5> {
        M_RST_W::new(self)
    }
    #[doc = "Bit 6 - Master\\[i\\] Reset\n\nMaster\\[i\\] address convert lane software reset switch.\n\nWhen Master\\[i\\] occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    #[must_use]
    pub fn m6_rst(&mut self) -> M_RST_W<6> {
        M_RST_W::new(self)
    }
    #[doc = "Bit 16 - Macrotlb Reset\n\nMacro TLB address convert lane software reset switch.\n\nWhen PTW Cache occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    #[must_use]
    pub fn mtlb_rst(&mut self) -> MTLB_RST_W<16> {
        MTLB_RST_W::new(self)
    }
    #[doc = "Bit 17 - PTW Cache Reset\n\nPTW Cache address convert lane software reset switch.\n\nWhen PTW Cache occurs abnormal, the bit is used to reset PTW Cache individually."]
    #[inline(always)]
    #[must_use]
    pub fn pc_rst(&mut self) -> PC_RST_W<17> {
        PC_RST_W::new(self)
    }
    #[doc = "Bit 31 - IOMMU Software Reset Switch、n\nBefore IOMMU software reset operation, ensure IOMMU never be opened; or all bus operations are completed; or DRAM and the peripherals have opened the corresponding switch, for shielding the effects of IOMMU reset."]
    #[inline(always)]
    #[must_use]
    pub fn iommu_reset(&mut self) -> IOMMU_RESET_W<31> {
        IOMMU_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMMU Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_reset](index.html) module"]
pub struct IOMMU_RESET_SPEC;
impl crate::RegisterSpec for IOMMU_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_reset::R](R) reader structure"]
impl crate::Readable for IOMMU_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iommu_reset::W](W) writer structure"]
impl crate::Writable for IOMMU_RESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_reset to value 0x8003_007f"]
impl crate::Resettable for IOMMU_RESET_SPEC {
    const RESET_VALUE: Self::Ux = 0x8003_007f;
}
