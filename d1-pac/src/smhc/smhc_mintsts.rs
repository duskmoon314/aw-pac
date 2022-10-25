#[doc = "Register `smhc_mintsts` reader"]
pub struct R(crate::R<SMHC_MINTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_MINTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_MINTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_MINTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `m_re_int` reader - Response Errors"]
pub type M_RE_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_cc_int` reader - Command Complete"]
pub type M_CC_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_dtc_int` reader - Data Transfer Complete"]
pub type M_DTC_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_dtr_int` reader - Data Transmit Request"]
pub type M_DTR_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_drr_int` reader - Data Receive Request"]
pub type M_DRR_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_rce_int` reader - Response CRC Error"]
pub type M_RCE_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_dce_int` reader - Data CRC Error"]
pub type M_DCE_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_rto_back_int` reader - Response Timeout/Boot ACK Received"]
pub type M_RTO_BACK_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_dto_bds_int` reader - Data Timeout/Boot Data Start"]
pub type M_DTO_BDS_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_dsto_vsd_int` reader - Data Starvation Timeout/V1.8 Switch Done"]
pub type M_DSTO_VSD_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_fu_fo_int` reader - FIFO Underrun/Overflow"]
pub type M_FU_FO_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_cb_iw_int` reader - Command Busy and Illegal Write"]
pub type M_CB_IW_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_dse_bc_int` reader - Data Start Error/Busy Clear"]
pub type M_DSE_BC_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_acd_int` reader - Auto Command Done"]
pub type M_ACD_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_dee_int` reader - Data End-bit Error"]
pub type M_DEE_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_sdio_int` reader - SDIO Interrupt"]
pub type M_SDIO_INT_R = crate::BitReader<bool>;
#[doc = "Field `m_card_insert` reader - Card Inserted"]
pub type M_CARD_INSERT_R = crate::BitReader<bool>;
#[doc = "Field `m_card_removal_int` reader - Card Removed"]
pub type M_CARD_REMOVAL_INT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Response Errors"]
    #[inline(always)]
    pub fn m_re_int(&self) -> M_RE_INT_R {
        M_RE_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command Complete"]
    #[inline(always)]
    pub fn m_cc_int(&self) -> M_CC_INT_R {
        M_CC_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Transfer Complete"]
    #[inline(always)]
    pub fn m_dtc_int(&self) -> M_DTC_INT_R {
        M_DTC_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Transmit Request"]
    #[inline(always)]
    pub fn m_dtr_int(&self) -> M_DTR_INT_R {
        M_DTR_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Receive Request"]
    #[inline(always)]
    pub fn m_drr_int(&self) -> M_DRR_INT_R {
        M_DRR_INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Response CRC Error"]
    #[inline(always)]
    pub fn m_rce_int(&self) -> M_RCE_INT_R {
        M_RCE_INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data CRC Error"]
    #[inline(always)]
    pub fn m_dce_int(&self) -> M_DCE_INT_R {
        M_DCE_INT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Response Timeout/Boot ACK Received"]
    #[inline(always)]
    pub fn m_rto_back_int(&self) -> M_RTO_BACK_INT_R {
        M_RTO_BACK_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Timeout/Boot Data Start"]
    #[inline(always)]
    pub fn m_dto_bds_int(&self) -> M_DTO_BDS_INT_R {
        M_DTO_BDS_INT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Starvation Timeout/V1.8 Switch Done"]
    #[inline(always)]
    pub fn m_dsto_vsd_int(&self) -> M_DSTO_VSD_INT_R {
        M_DSTO_VSD_INT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FIFO Underrun/Overflow"]
    #[inline(always)]
    pub fn m_fu_fo_int(&self) -> M_FU_FO_INT_R {
        M_FU_FO_INT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Command Busy and Illegal Write"]
    #[inline(always)]
    pub fn m_cb_iw_int(&self) -> M_CB_IW_INT_R {
        M_CB_IW_INT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data Start Error/Busy Clear"]
    #[inline(always)]
    pub fn m_dse_bc_int(&self) -> M_DSE_BC_INT_R {
        M_DSE_BC_INT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto Command Done"]
    #[inline(always)]
    pub fn m_acd_int(&self) -> M_ACD_INT_R {
        M_ACD_INT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data End-bit Error"]
    #[inline(always)]
    pub fn m_dee_int(&self) -> M_DEE_INT_R {
        M_DEE_INT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SDIO Interrupt"]
    #[inline(always)]
    pub fn m_sdio_int(&self) -> M_SDIO_INT_R {
        M_SDIO_INT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 30 - Card Inserted"]
    #[inline(always)]
    pub fn m_card_insert(&self) -> M_CARD_INSERT_R {
        M_CARD_INSERT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Card Removed"]
    #[inline(always)]
    pub fn m_card_removal_int(&self) -> M_CARD_REMOVAL_INT_R {
        M_CARD_REMOVAL_INT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Masked Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_mintsts](index.html) module"]
pub struct SMHC_MINTSTS_SPEC;
impl crate::RegisterSpec for SMHC_MINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_mintsts::R](R) reader structure"]
impl crate::Readable for SMHC_MINTSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets smhc_mintsts to value 0"]
impl crate::Resettable for SMHC_MINTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
