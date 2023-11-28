#[doc = "Register `phy_status` reader"]
pub type R = crate::R<PHY_STATUS_SPEC>;
#[doc = "Field `vc_do` reader - vc_do"]
pub type VC_DO_R = crate::BitReader;
#[doc = "Field `bist_done` reader - bist_done"]
pub type BIST_DONE_R = crate::BitReader;
#[doc = "Field `bist_error` reader - Bist_error"]
pub type BIST_ERROR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - vc_do"]
    #[inline(always)]
    pub fn vc_do(&self) -> VC_DO_R {
        VC_DO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - bist_done"]
    #[inline(always)]
    pub fn bist_done(&self) -> BIST_DONE_R {
        BIST_DONE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bist_error"]
    #[inline(always)]
    pub fn bist_error(&self) -> BIST_ERROR_R {
        BIST_ERROR_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "PHY Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_STATUS_SPEC;
impl crate::RegisterSpec for PHY_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_status::R`](R) reader structure"]
impl crate::Readable for PHY_STATUS_SPEC {}
#[doc = "`reset()` method sets phy_status to value 0"]
impl crate::Resettable for PHY_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
