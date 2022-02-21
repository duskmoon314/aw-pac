#[doc = "Register `SMHC_MINTSTS` reader"]
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
#[doc = "Field `M_CARD_REMOVAL_INT` reader - Card Removed"]
pub struct M_CARD_REMOVAL_INT_R(crate::FieldReader<bool, bool>);
impl M_CARD_REMOVAL_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_CARD_REMOVAL_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_CARD_REMOVAL_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_CARD_INSERT` reader - Card Inserted"]
pub struct M_CARD_INSERT_R(crate::FieldReader<bool, bool>);
impl M_CARD_INSERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_CARD_INSERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_CARD_INSERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_SDIO_INT` reader - SDIO Interrupt"]
pub struct M_SDIO_INT_R(crate::FieldReader<bool, bool>);
impl M_SDIO_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_SDIO_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_SDIO_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_DEE_INT` reader - Data End-bit Error"]
pub struct M_DEE_INT_R(crate::FieldReader<bool, bool>);
impl M_DEE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_DEE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_DEE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_ACD_INT` reader - Auto Command Done"]
pub struct M_ACD_INT_R(crate::FieldReader<bool, bool>);
impl M_ACD_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_ACD_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_ACD_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_DSE_BC_INT` reader - Data Start Error/Busy Clear"]
pub struct M_DSE_BC_INT_R(crate::FieldReader<bool, bool>);
impl M_DSE_BC_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_DSE_BC_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_DSE_BC_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_CB_IW_INT` reader - Command Busy and Illegal Write"]
pub struct M_CB_IW_INT_R(crate::FieldReader<bool, bool>);
impl M_CB_IW_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_CB_IW_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_CB_IW_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_FU_FO_INT` reader - FIFO Underrun/Overflow"]
pub struct M_FU_FO_INT_R(crate::FieldReader<bool, bool>);
impl M_FU_FO_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_FU_FO_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_FU_FO_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_DSTO_VSD_INT` reader - Data Starvation Timeout/V1.8 Switch Done"]
pub struct M_DSTO_VSD_INT_R(crate::FieldReader<bool, bool>);
impl M_DSTO_VSD_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_DSTO_VSD_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_DSTO_VSD_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_DTO_BDS_INT` reader - Data Timeout/Boot Data Start"]
pub struct M_DTO_BDS_INT_R(crate::FieldReader<bool, bool>);
impl M_DTO_BDS_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_DTO_BDS_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_DTO_BDS_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_RTO_BACK_INT` reader - Response Timeout/Boot ACK Received"]
pub struct M_RTO_BACK_INT_R(crate::FieldReader<bool, bool>);
impl M_RTO_BACK_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_RTO_BACK_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_RTO_BACK_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_DCE_INT` reader - Data CRC Error"]
pub struct M_DCE_INT_R(crate::FieldReader<bool, bool>);
impl M_DCE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_DCE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_DCE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_RCE_INT` reader - Response CRC Error"]
pub struct M_RCE_INT_R(crate::FieldReader<bool, bool>);
impl M_RCE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_RCE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_RCE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_DRR_INT` reader - Data Receive Request"]
pub struct M_DRR_INT_R(crate::FieldReader<bool, bool>);
impl M_DRR_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_DRR_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_DRR_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_DTR_INT` reader - Data Transmit Request"]
pub struct M_DTR_INT_R(crate::FieldReader<bool, bool>);
impl M_DTR_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_DTR_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_DTR_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_DTC_INT` reader - Data Transfer Complete"]
pub struct M_DTC_INT_R(crate::FieldReader<bool, bool>);
impl M_DTC_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_DTC_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_DTC_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_CC_INT` reader - Command Complete"]
pub struct M_CC_INT_R(crate::FieldReader<bool, bool>);
impl M_CC_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_CC_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_CC_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_RE_INT` reader - Response Errors"]
pub struct M_RE_INT_R(crate::FieldReader<bool, bool>);
impl M_RE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M_RE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_RE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31 - Card Removed"]
    #[inline(always)]
    pub fn m_card_removal_int(&self) -> M_CARD_REMOVAL_INT_R {
        M_CARD_REMOVAL_INT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Card Inserted"]
    #[inline(always)]
    pub fn m_card_insert(&self) -> M_CARD_INSERT_R {
        M_CARD_INSERT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDIO Interrupt"]
    #[inline(always)]
    pub fn m_sdio_int(&self) -> M_SDIO_INT_R {
        M_SDIO_INT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Data End-bit Error"]
    #[inline(always)]
    pub fn m_dee_int(&self) -> M_DEE_INT_R {
        M_DEE_INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Auto Command Done"]
    #[inline(always)]
    pub fn m_acd_int(&self) -> M_ACD_INT_R {
        M_ACD_INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Data Start Error/Busy Clear"]
    #[inline(always)]
    pub fn m_dse_bc_int(&self) -> M_DSE_BC_INT_R {
        M_DSE_BC_INT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Command Busy and Illegal Write"]
    #[inline(always)]
    pub fn m_cb_iw_int(&self) -> M_CB_IW_INT_R {
        M_CB_IW_INT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - FIFO Underrun/Overflow"]
    #[inline(always)]
    pub fn m_fu_fo_int(&self) -> M_FU_FO_INT_R {
        M_FU_FO_INT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data Starvation Timeout/V1.8 Switch Done"]
    #[inline(always)]
    pub fn m_dsto_vsd_int(&self) -> M_DSTO_VSD_INT_R {
        M_DSTO_VSD_INT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data Timeout/Boot Data Start"]
    #[inline(always)]
    pub fn m_dto_bds_int(&self) -> M_DTO_BDS_INT_R {
        M_DTO_BDS_INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Response Timeout/Boot ACK Received"]
    #[inline(always)]
    pub fn m_rto_back_int(&self) -> M_RTO_BACK_INT_R {
        M_RTO_BACK_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Data CRC Error"]
    #[inline(always)]
    pub fn m_dce_int(&self) -> M_DCE_INT_R {
        M_DCE_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Response CRC Error"]
    #[inline(always)]
    pub fn m_rce_int(&self) -> M_RCE_INT_R {
        M_RCE_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data Receive Request"]
    #[inline(always)]
    pub fn m_drr_int(&self) -> M_DRR_INT_R {
        M_DRR_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Transmit Request"]
    #[inline(always)]
    pub fn m_dtr_int(&self) -> M_DTR_INT_R {
        M_DTR_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data Transfer Complete"]
    #[inline(always)]
    pub fn m_dtc_int(&self) -> M_DTC_INT_R {
        M_DTC_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command Complete"]
    #[inline(always)]
    pub fn m_cc_int(&self) -> M_CC_INT_R {
        M_CC_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Response Errors"]
    #[inline(always)]
    pub fn m_re_int(&self) -> M_RE_INT_R {
        M_RE_INT_R::new(((self.bits >> 1) & 0x01) != 0)
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
#[doc = "`reset()` method sets SMHC_MINTSTS to value 0"]
impl crate::Resettable for SMHC_MINTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
