#[doc = "Register `SMHC_RINTSTS` reader"]
pub struct R(crate::R<SMHC_RINTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_RINTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_RINTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_RINTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_RINTSTS` writer"]
pub struct W(crate::W<SMHC_RINTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_RINTSTS_SPEC>;
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
impl From<crate::W<SMHC_RINTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_RINTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARD_REMOVAL` reader - Card Removed"]
pub type CARD_REMOVAL_R = crate::BitReader<bool>;
#[doc = "Field `CARD_REMOVAL` writer - Card Removed"]
pub type CARD_REMOVAL_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 31>;
#[doc = "Field `CARD_INSERT` reader - Card Inserted"]
pub type CARD_INSERT_R = crate::BitReader<bool>;
#[doc = "Field `CARD_INSERT` writer - Card Inserted"]
pub type CARD_INSERT_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 30>;
#[doc = "Field `SDIOI_INT` reader - SDIO Interrupt"]
pub type SDIOI_INT_R = crate::BitReader<bool>;
#[doc = "Field `SDIOI_INT` writer - SDIO Interrupt"]
pub type SDIOI_INT_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 16>;
#[doc = "Field `DEE` reader - Data End-bit Error"]
pub type DEE_R = crate::BitReader<bool>;
#[doc = "Field `DEE` writer - Data End-bit Error"]
pub type DEE_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 15>;
#[doc = "Field `ACD` reader - Auto Command Done"]
pub type ACD_R = crate::BitReader<bool>;
#[doc = "Field `ACD` writer - Auto Command Done"]
pub type ACD_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 14>;
#[doc = "Field `DSE_BC` reader - Data Start Error/Busy Clear"]
pub type DSE_BC_R = crate::BitReader<bool>;
#[doc = "Field `DSE_BC` writer - Data Start Error/Busy Clear"]
pub type DSE_BC_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 13>;
#[doc = "Field `CB_IW` reader - Command Busy and Illegal Write"]
pub type CB_IW_R = crate::BitReader<bool>;
#[doc = "Field `CB_IW` writer - Command Busy and Illegal Write"]
pub type CB_IW_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 12>;
#[doc = "Field `FU_FO` reader - FIFO Underrun/Overflow"]
pub type FU_FO_R = crate::BitReader<bool>;
#[doc = "Field `FU_FO` writer - FIFO Underrun/Overflow"]
pub type FU_FO_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 11>;
#[doc = "Field `DSTO_VSD` reader - Data Starvation Timeout/V1.8 Switch Done"]
pub type DSTO_VSD_R = crate::BitReader<bool>;
#[doc = "Field `DSTO_VSD` writer - Data Starvation Timeout/V1.8 Switch Done"]
pub type DSTO_VSD_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 10>;
#[doc = "Field `DTO_BDS` reader - Data Timeout/Boot Data Start"]
pub type DTO_BDS_R = crate::BitReader<bool>;
#[doc = "Field `DTO_BDS` writer - Data Timeout/Boot Data Start"]
pub type DTO_BDS_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 9>;
#[doc = "Field `RTO_BACK` reader - Response Timeout/Boot ACK Received"]
pub type RTO_BACK_R = crate::BitReader<bool>;
#[doc = "Field `RTO_BACK` writer - Response Timeout/Boot ACK Received"]
pub type RTO_BACK_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 8>;
#[doc = "Field `DCE` reader - Data CRC Error"]
pub type DCE_R = crate::BitReader<bool>;
#[doc = "Field `DCE` writer - Data CRC Error"]
pub type DCE_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 7>;
#[doc = "Field `RCE` reader - Response CRC Error"]
pub type RCE_R = crate::BitReader<bool>;
#[doc = "Field `RCE` writer - Response CRC Error"]
pub type RCE_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 6>;
#[doc = "Field `DRR` reader - Data Receive Request"]
pub type DRR_R = crate::BitReader<bool>;
#[doc = "Field `DRR` writer - Data Receive Request"]
pub type DRR_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 5>;
#[doc = "Field `DTR` reader - Data Transmit Request"]
pub type DTR_R = crate::BitReader<bool>;
#[doc = "Field `DTR` writer - Data Transmit Request"]
pub type DTR_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 4>;
#[doc = "Field `DTC` reader - Data Transfer Complete"]
pub type DTC_R = crate::BitReader<bool>;
#[doc = "Field `DTC` writer - Data Transfer Complete"]
pub type DTC_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 3>;
#[doc = "Field `CC` reader - Command Complete"]
pub type CC_R = crate::BitReader<bool>;
#[doc = "Field `CC` writer - Command Complete"]
pub type CC_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 2>;
#[doc = "Field `RE` reader - Response Error"]
pub type RE_R = crate::BitReader<bool>;
#[doc = "Field `RE` writer - Response Error"]
pub type RE_W<'a> = crate::BitWriter<'a, u32, SMHC_RINTSTS_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 31 - Card Removed"]
    #[inline(always)]
    pub fn card_removal(&self) -> CARD_REMOVAL_R {
        CARD_REMOVAL_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Card Inserted"]
    #[inline(always)]
    pub fn card_insert(&self) -> CARD_INSERT_R {
        CARD_INSERT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 16 - SDIO Interrupt"]
    #[inline(always)]
    pub fn sdioi_int(&self) -> SDIOI_INT_R {
        SDIOI_INT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Data End-bit Error"]
    #[inline(always)]
    pub fn dee(&self) -> DEE_R {
        DEE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto Command Done"]
    #[inline(always)]
    pub fn acd(&self) -> ACD_R {
        ACD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Data Start Error/Busy Clear"]
    #[inline(always)]
    pub fn dse_bc(&self) -> DSE_BC_R {
        DSE_BC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Command Busy and Illegal Write"]
    #[inline(always)]
    pub fn cb_iw(&self) -> CB_IW_R {
        CB_IW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - FIFO Underrun/Overflow"]
    #[inline(always)]
    pub fn fu_fo(&self) -> FU_FO_R {
        FU_FO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Starvation Timeout/V1.8 Switch Done"]
    #[inline(always)]
    pub fn dsto_vsd(&self) -> DSTO_VSD_R {
        DSTO_VSD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Timeout/Boot Data Start"]
    #[inline(always)]
    pub fn dto_bds(&self) -> DTO_BDS_R {
        DTO_BDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Response Timeout/Boot ACK Received"]
    #[inline(always)]
    pub fn rto_back(&self) -> RTO_BACK_R {
        RTO_BACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Data CRC Error"]
    #[inline(always)]
    pub fn dce(&self) -> DCE_R {
        DCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Response CRC Error"]
    #[inline(always)]
    pub fn rce(&self) -> RCE_R {
        RCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Receive Request"]
    #[inline(always)]
    pub fn drr(&self) -> DRR_R {
        DRR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Transmit Request"]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Transfer Complete"]
    #[inline(always)]
    pub fn dtc(&self) -> DTC_R {
        DTC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Command Complete"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Response Error"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Card Removed"]
    #[inline(always)]
    pub fn card_removal(&mut self) -> CARD_REMOVAL_W {
        CARD_REMOVAL_W::new(self)
    }
    #[doc = "Bit 30 - Card Inserted"]
    #[inline(always)]
    pub fn card_insert(&mut self) -> CARD_INSERT_W {
        CARD_INSERT_W::new(self)
    }
    #[doc = "Bit 16 - SDIO Interrupt"]
    #[inline(always)]
    pub fn sdioi_int(&mut self) -> SDIOI_INT_W {
        SDIOI_INT_W::new(self)
    }
    #[doc = "Bit 15 - Data End-bit Error"]
    #[inline(always)]
    pub fn dee(&mut self) -> DEE_W {
        DEE_W::new(self)
    }
    #[doc = "Bit 14 - Auto Command Done"]
    #[inline(always)]
    pub fn acd(&mut self) -> ACD_W {
        ACD_W::new(self)
    }
    #[doc = "Bit 13 - Data Start Error/Busy Clear"]
    #[inline(always)]
    pub fn dse_bc(&mut self) -> DSE_BC_W {
        DSE_BC_W::new(self)
    }
    #[doc = "Bit 12 - Command Busy and Illegal Write"]
    #[inline(always)]
    pub fn cb_iw(&mut self) -> CB_IW_W {
        CB_IW_W::new(self)
    }
    #[doc = "Bit 11 - FIFO Underrun/Overflow"]
    #[inline(always)]
    pub fn fu_fo(&mut self) -> FU_FO_W {
        FU_FO_W::new(self)
    }
    #[doc = "Bit 10 - Data Starvation Timeout/V1.8 Switch Done"]
    #[inline(always)]
    pub fn dsto_vsd(&mut self) -> DSTO_VSD_W {
        DSTO_VSD_W::new(self)
    }
    #[doc = "Bit 9 - Data Timeout/Boot Data Start"]
    #[inline(always)]
    pub fn dto_bds(&mut self) -> DTO_BDS_W {
        DTO_BDS_W::new(self)
    }
    #[doc = "Bit 8 - Response Timeout/Boot ACK Received"]
    #[inline(always)]
    pub fn rto_back(&mut self) -> RTO_BACK_W {
        RTO_BACK_W::new(self)
    }
    #[doc = "Bit 7 - Data CRC Error"]
    #[inline(always)]
    pub fn dce(&mut self) -> DCE_W {
        DCE_W::new(self)
    }
    #[doc = "Bit 6 - Response CRC Error"]
    #[inline(always)]
    pub fn rce(&mut self) -> RCE_W {
        RCE_W::new(self)
    }
    #[doc = "Bit 5 - Data Receive Request"]
    #[inline(always)]
    pub fn drr(&mut self) -> DRR_W {
        DRR_W::new(self)
    }
    #[doc = "Bit 4 - Data Transmit Request"]
    #[inline(always)]
    pub fn dtr(&mut self) -> DTR_W {
        DTR_W::new(self)
    }
    #[doc = "Bit 3 - Data Transfer Complete"]
    #[inline(always)]
    pub fn dtc(&mut self) -> DTC_W {
        DTC_W::new(self)
    }
    #[doc = "Bit 2 - Command Complete"]
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W {
        CC_W::new(self)
    }
    #[doc = "Bit 1 - Response Error"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_rintsts](index.html) module"]
pub struct SMHC_RINTSTS_SPEC;
impl crate::RegisterSpec for SMHC_RINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_rintsts::R](R) reader structure"]
impl crate::Readable for SMHC_RINTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_rintsts::W](W) writer structure"]
impl crate::Writable for SMHC_RINTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_RINTSTS to value 0"]
impl crate::Resettable for SMHC_RINTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
