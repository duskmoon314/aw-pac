#[doc = "Register `SMHC_INTMASK` reader"]
pub struct R(crate::R<SMHC_INTMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_INTMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_INTMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_INTMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_INTMASK` writer"]
pub struct W(crate::W<SMHC_INTMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_INTMASK_SPEC>;
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
impl From<crate::W<SMHC_INTMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_INTMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARD_REMOVAL_INT_EN` reader - Card Removed Interrupt Enable"]
pub type CARD_REMOVAL_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CARD_REMOVAL_INT_EN` writer - Card Removed Interrupt Enable"]
pub type CARD_REMOVAL_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 31>;
#[doc = "Field `CARD_INSERT_INT_EN` reader - Card Inserted Interrupt Enable"]
pub type CARD_INSERT_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CARD_INSERT_INT_EN` writer - Card Inserted Interrupt Enable"]
pub type CARD_INSERT_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 30>;
#[doc = "Field `SDIO_INT_EN` reader - SDIO Interrupt Enable"]
pub type SDIO_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_INT_EN` writer - SDIO Interrupt Enable"]
pub type SDIO_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 16>;
#[doc = "Field `DEE_INT_EN` reader - Data End-bit Error Interrupt Enable"]
pub type DEE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DEE_INT_EN` writer - Data End-bit Error Interrupt Enable"]
pub type DEE_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 15>;
#[doc = "Field `ACD_INT_EN` reader - Auto Command Done Interrupt Enable"]
pub type ACD_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `ACD_INT_EN` writer - Auto Command Done Interrupt Enable"]
pub type ACD_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 14>;
#[doc = "Field `DSE_BC_INT_EN` reader - Data Start Error Interrupt Enable"]
pub type DSE_BC_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DSE_BC_INT_EN` writer - Data Start Error Interrupt Enable"]
pub type DSE_BC_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 13>;
#[doc = "Field `CB_IW_INT_EN` reader - Command Busy and Illegal Write Interrupt Enable"]
pub type CB_IW_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CB_IW_INT_EN` writer - Command Busy and Illegal Write Interrupt Enable"]
pub type CB_IW_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 12>;
#[doc = "Field `FU_FO_INT_EN` reader - FIFO Underrun/Overflow Interrupt Enable"]
pub type FU_FO_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `FU_FO_INT_EN` writer - FIFO Underrun/Overflow Interrupt Enable"]
pub type FU_FO_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 11>;
#[doc = "Field `DSTO_VSD_INT_EN` reader - Data Starvation Timeout/V1.8 Switch Done Interrupt Enable"]
pub type DSTO_VSD_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DSTO_VSD_INT_EN` writer - Data Starvation Timeout/V1.8 Switch Done Interrupt Enable"]
pub type DSTO_VSD_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 10>;
#[doc = "Field `DTO_BDS_INT_EN` reader - Data Timeout/Boot Data Start Interrupt Enable"]
pub type DTO_BDS_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DTO_BDS_INT_EN` writer - Data Timeout/Boot Data Start Interrupt Enable"]
pub type DTO_BDS_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 9>;
#[doc = "Field `RTO_BACK_INT_EN` reader - Response Timeout/Boot ACK Received Interrupt Enable"]
pub type RTO_BACK_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RTO_BACK_INT_EN` writer - Response Timeout/Boot ACK Received Interrupt Enable"]
pub type RTO_BACK_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 8>;
#[doc = "Field `DCE_INT_EN` reader - Data CRC Error Interrupt Enable"]
pub type DCE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DCE_INT_EN` writer - Data CRC Error Interrupt Enable"]
pub type DCE_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 7>;
#[doc = "Field `RCE_INT_EN` reader - Response CRC Error Interrupt Enable"]
pub type RCE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RCE_INT_EN` writer - Response CRC Error Interrupt Enable"]
pub type RCE_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 6>;
#[doc = "Field `DRR_INT_EN` reader - Data Receive Request Interrupt Enable"]
pub type DRR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DRR_INT_EN` writer - Data Receive Request Interrupt Enable"]
pub type DRR_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 5>;
#[doc = "Field `DTR_INT_EN` reader - Data Transmit Request Interrupt Enable"]
pub type DTR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DTR_INT_EN` writer - Data Transmit Request Interrupt Enable"]
pub type DTR_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 4>;
#[doc = "Field `DTC_INT_EN` reader - Data Transfer Complete Interrupt Enable"]
pub type DTC_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DTC_INT_EN` writer - Data Transfer Complete Interrupt Enable"]
pub type DTC_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 3>;
#[doc = "Field `CC_INT_EN` reader - Command Complete Interrupt Enable"]
pub type CC_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CC_INT_EN` writer - Command Complete Interrupt Enable"]
pub type CC_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 2>;
#[doc = "Field `RE_INT_EN` reader - Response Error Interrupt Enable"]
pub type RE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RE_INT_EN` writer - Response Error Interrupt Enable"]
pub type RE_INT_EN_W<'a> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 31 - Card Removed Interrupt Enable"]
    #[inline(always)]
    pub fn card_removal_int_en(&self) -> CARD_REMOVAL_INT_EN_R {
        CARD_REMOVAL_INT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Card Inserted Interrupt Enable"]
    #[inline(always)]
    pub fn card_insert_int_en(&self) -> CARD_INSERT_INT_EN_R {
        CARD_INSERT_INT_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 16 - SDIO Interrupt Enable"]
    #[inline(always)]
    pub fn sdio_int_en(&self) -> SDIO_INT_EN_R {
        SDIO_INT_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Data End-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn dee_int_en(&self) -> DEE_INT_EN_R {
        DEE_INT_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto Command Done Interrupt Enable"]
    #[inline(always)]
    pub fn acd_int_en(&self) -> ACD_INT_EN_R {
        ACD_INT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Data Start Error Interrupt Enable"]
    #[inline(always)]
    pub fn dse_bc_int_en(&self) -> DSE_BC_INT_EN_R {
        DSE_BC_INT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Command Busy and Illegal Write Interrupt Enable"]
    #[inline(always)]
    pub fn cb_iw_int_en(&self) -> CB_IW_INT_EN_R {
        CB_IW_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - FIFO Underrun/Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fu_fo_int_en(&self) -> FU_FO_INT_EN_R {
        FU_FO_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Starvation Timeout/V1.8 Switch Done Interrupt Enable"]
    #[inline(always)]
    pub fn dsto_vsd_int_en(&self) -> DSTO_VSD_INT_EN_R {
        DSTO_VSD_INT_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Timeout/Boot Data Start Interrupt Enable"]
    #[inline(always)]
    pub fn dto_bds_int_en(&self) -> DTO_BDS_INT_EN_R {
        DTO_BDS_INT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Response Timeout/Boot ACK Received Interrupt Enable"]
    #[inline(always)]
    pub fn rto_back_int_en(&self) -> RTO_BACK_INT_EN_R {
        RTO_BACK_INT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Data CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn dce_int_en(&self) -> DCE_INT_EN_R {
        DCE_INT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Response CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn rce_int_en(&self) -> RCE_INT_EN_R {
        RCE_INT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Receive Request Interrupt Enable"]
    #[inline(always)]
    pub fn drr_int_en(&self) -> DRR_INT_EN_R {
        DRR_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Transmit Request Interrupt Enable"]
    #[inline(always)]
    pub fn dtr_int_en(&self) -> DTR_INT_EN_R {
        DTR_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dtc_int_en(&self) -> DTC_INT_EN_R {
        DTC_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn cc_int_en(&self) -> CC_INT_EN_R {
        CC_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Response Error Interrupt Enable"]
    #[inline(always)]
    pub fn re_int_en(&self) -> RE_INT_EN_R {
        RE_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Card Removed Interrupt Enable"]
    #[inline(always)]
    pub fn card_removal_int_en(&mut self) -> CARD_REMOVAL_INT_EN_W {
        CARD_REMOVAL_INT_EN_W::new(self)
    }
    #[doc = "Bit 30 - Card Inserted Interrupt Enable"]
    #[inline(always)]
    pub fn card_insert_int_en(&mut self) -> CARD_INSERT_INT_EN_W {
        CARD_INSERT_INT_EN_W::new(self)
    }
    #[doc = "Bit 16 - SDIO Interrupt Enable"]
    #[inline(always)]
    pub fn sdio_int_en(&mut self) -> SDIO_INT_EN_W {
        SDIO_INT_EN_W::new(self)
    }
    #[doc = "Bit 15 - Data End-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn dee_int_en(&mut self) -> DEE_INT_EN_W {
        DEE_INT_EN_W::new(self)
    }
    #[doc = "Bit 14 - Auto Command Done Interrupt Enable"]
    #[inline(always)]
    pub fn acd_int_en(&mut self) -> ACD_INT_EN_W {
        ACD_INT_EN_W::new(self)
    }
    #[doc = "Bit 13 - Data Start Error Interrupt Enable"]
    #[inline(always)]
    pub fn dse_bc_int_en(&mut self) -> DSE_BC_INT_EN_W {
        DSE_BC_INT_EN_W::new(self)
    }
    #[doc = "Bit 12 - Command Busy and Illegal Write Interrupt Enable"]
    #[inline(always)]
    pub fn cb_iw_int_en(&mut self) -> CB_IW_INT_EN_W {
        CB_IW_INT_EN_W::new(self)
    }
    #[doc = "Bit 11 - FIFO Underrun/Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fu_fo_int_en(&mut self) -> FU_FO_INT_EN_W {
        FU_FO_INT_EN_W::new(self)
    }
    #[doc = "Bit 10 - Data Starvation Timeout/V1.8 Switch Done Interrupt Enable"]
    #[inline(always)]
    pub fn dsto_vsd_int_en(&mut self) -> DSTO_VSD_INT_EN_W {
        DSTO_VSD_INT_EN_W::new(self)
    }
    #[doc = "Bit 9 - Data Timeout/Boot Data Start Interrupt Enable"]
    #[inline(always)]
    pub fn dto_bds_int_en(&mut self) -> DTO_BDS_INT_EN_W {
        DTO_BDS_INT_EN_W::new(self)
    }
    #[doc = "Bit 8 - Response Timeout/Boot ACK Received Interrupt Enable"]
    #[inline(always)]
    pub fn rto_back_int_en(&mut self) -> RTO_BACK_INT_EN_W {
        RTO_BACK_INT_EN_W::new(self)
    }
    #[doc = "Bit 7 - Data CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn dce_int_en(&mut self) -> DCE_INT_EN_W {
        DCE_INT_EN_W::new(self)
    }
    #[doc = "Bit 6 - Response CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn rce_int_en(&mut self) -> RCE_INT_EN_W {
        RCE_INT_EN_W::new(self)
    }
    #[doc = "Bit 5 - Data Receive Request Interrupt Enable"]
    #[inline(always)]
    pub fn drr_int_en(&mut self) -> DRR_INT_EN_W {
        DRR_INT_EN_W::new(self)
    }
    #[doc = "Bit 4 - Data Transmit Request Interrupt Enable"]
    #[inline(always)]
    pub fn dtr_int_en(&mut self) -> DTR_INT_EN_W {
        DTR_INT_EN_W::new(self)
    }
    #[doc = "Bit 3 - Data Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dtc_int_en(&mut self) -> DTC_INT_EN_W {
        DTC_INT_EN_W::new(self)
    }
    #[doc = "Bit 2 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn cc_int_en(&mut self) -> CC_INT_EN_W {
        CC_INT_EN_W::new(self)
    }
    #[doc = "Bit 1 - Response Error Interrupt Enable"]
    #[inline(always)]
    pub fn re_int_en(&mut self) -> RE_INT_EN_W {
        RE_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_intmask](index.html) module"]
pub struct SMHC_INTMASK_SPEC;
impl crate::RegisterSpec for SMHC_INTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_intmask::R](R) reader structure"]
impl crate::Readable for SMHC_INTMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_intmask::W](W) writer structure"]
impl crate::Writable for SMHC_INTMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_INTMASK to value 0"]
impl crate::Resettable for SMHC_INTMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
