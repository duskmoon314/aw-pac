#[doc = "Register `smhc_intmask` reader"]
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
#[doc = "Register `smhc_intmask` writer"]
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
#[doc = "Field `re_int_en` reader - Response Error Interrupt Enable"]
pub type RE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `re_int_en` writer - Response Error Interrupt Enable"]
pub type RE_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `cc_int_en` reader - Command Complete Interrupt Enable"]
pub type CC_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cc_int_en` writer - Command Complete Interrupt Enable"]
pub type CC_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `dtc_int_en` reader - Data Transfer Complete Interrupt Enable"]
pub type DTC_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `dtc_int_en` writer - Data Transfer Complete Interrupt Enable"]
pub type DTC_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `dtr_int_en` reader - Data Transmit Request Interrupt Enable"]
pub type DTR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `dtr_int_en` writer - Data Transmit Request Interrupt Enable"]
pub type DTR_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `drr_int_en` reader - Data Receive Request Interrupt Enable"]
pub type DRR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `drr_int_en` writer - Data Receive Request Interrupt Enable"]
pub type DRR_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `rce_int_en` reader - Response CRC Error Interrupt Enable"]
pub type RCE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `rce_int_en` writer - Response CRC Error Interrupt Enable"]
pub type RCE_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `dce_int_en` reader - Data CRC Error Interrupt Enable"]
pub type DCE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `dce_int_en` writer - Data CRC Error Interrupt Enable"]
pub type DCE_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `rto_back_int_en` reader - Response Timeout/Boot ACK Received Interrupt Enable"]
pub type RTO_BACK_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `rto_back_int_en` writer - Response Timeout/Boot ACK Received Interrupt Enable"]
pub type RTO_BACK_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `dto_bds_int_en` reader - Data Timeout/Boot Data Start Interrupt Enable"]
pub type DTO_BDS_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `dto_bds_int_en` writer - Data Timeout/Boot Data Start Interrupt Enable"]
pub type DTO_BDS_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `dsto_vsd_int_en` reader - Data Starvation Timeout/V1.8 Switch Done Interrupt Enable"]
pub type DSTO_VSD_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `dsto_vsd_int_en` writer - Data Starvation Timeout/V1.8 Switch Done Interrupt Enable"]
pub type DSTO_VSD_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `fu_fo_int_en` reader - FIFO Underrun/Overflow Interrupt Enable"]
pub type FU_FO_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `fu_fo_int_en` writer - FIFO Underrun/Overflow Interrupt Enable"]
pub type FU_FO_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `cb_iw_int_en` reader - Command Busy and Illegal Write Interrupt Enable"]
pub type CB_IW_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cb_iw_int_en` writer - Command Busy and Illegal Write Interrupt Enable"]
pub type CB_IW_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `dse_bc_int_en` reader - Data Start Error Interrupt Enable"]
pub type DSE_BC_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `dse_bc_int_en` writer - Data Start Error Interrupt Enable"]
pub type DSE_BC_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `acd_int_en` reader - Auto Command Done Interrupt Enable"]
pub type ACD_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `acd_int_en` writer - Auto Command Done Interrupt Enable"]
pub type ACD_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `dee_int_en` reader - Data End-bit Error Interrupt Enable"]
pub type DEE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `dee_int_en` writer - Data End-bit Error Interrupt Enable"]
pub type DEE_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `sdio_int_en` reader - SDIO Interrupt Enable"]
pub type SDIO_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `sdio_int_en` writer - SDIO Interrupt Enable"]
pub type SDIO_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `card_insert_int_en` reader - Card Inserted Interrupt Enable"]
pub type CARD_INSERT_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `card_insert_int_en` writer - Card Inserted Interrupt Enable"]
pub type CARD_INSERT_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
#[doc = "Field `card_removal_int_en` reader - Card Removed Interrupt Enable"]
pub type CARD_REMOVAL_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `card_removal_int_en` writer - Card Removed Interrupt Enable"]
pub type CARD_REMOVAL_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SMHC_INTMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Response Error Interrupt Enable"]
    #[inline(always)]
    pub fn re_int_en(&self) -> RE_INT_EN_R {
        RE_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn cc_int_en(&self) -> CC_INT_EN_R {
        CC_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn dtc_int_en(&self) -> DTC_INT_EN_R {
        DTC_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Transmit Request Interrupt Enable"]
    #[inline(always)]
    pub fn dtr_int_en(&self) -> DTR_INT_EN_R {
        DTR_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Receive Request Interrupt Enable"]
    #[inline(always)]
    pub fn drr_int_en(&self) -> DRR_INT_EN_R {
        DRR_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Response CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn rce_int_en(&self) -> RCE_INT_EN_R {
        RCE_INT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn dce_int_en(&self) -> DCE_INT_EN_R {
        DCE_INT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Response Timeout/Boot ACK Received Interrupt Enable"]
    #[inline(always)]
    pub fn rto_back_int_en(&self) -> RTO_BACK_INT_EN_R {
        RTO_BACK_INT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Timeout/Boot Data Start Interrupt Enable"]
    #[inline(always)]
    pub fn dto_bds_int_en(&self) -> DTO_BDS_INT_EN_R {
        DTO_BDS_INT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Starvation Timeout/V1.8 Switch Done Interrupt Enable"]
    #[inline(always)]
    pub fn dsto_vsd_int_en(&self) -> DSTO_VSD_INT_EN_R {
        DSTO_VSD_INT_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FIFO Underrun/Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fu_fo_int_en(&self) -> FU_FO_INT_EN_R {
        FU_FO_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Command Busy and Illegal Write Interrupt Enable"]
    #[inline(always)]
    pub fn cb_iw_int_en(&self) -> CB_IW_INT_EN_R {
        CB_IW_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data Start Error Interrupt Enable"]
    #[inline(always)]
    pub fn dse_bc_int_en(&self) -> DSE_BC_INT_EN_R {
        DSE_BC_INT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto Command Done Interrupt Enable"]
    #[inline(always)]
    pub fn acd_int_en(&self) -> ACD_INT_EN_R {
        ACD_INT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data End-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn dee_int_en(&self) -> DEE_INT_EN_R {
        DEE_INT_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SDIO Interrupt Enable"]
    #[inline(always)]
    pub fn sdio_int_en(&self) -> SDIO_INT_EN_R {
        SDIO_INT_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 30 - Card Inserted Interrupt Enable"]
    #[inline(always)]
    pub fn card_insert_int_en(&self) -> CARD_INSERT_INT_EN_R {
        CARD_INSERT_INT_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Card Removed Interrupt Enable"]
    #[inline(always)]
    pub fn card_removal_int_en(&self) -> CARD_REMOVAL_INT_EN_R {
        CARD_REMOVAL_INT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Response Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn re_int_en(&mut self) -> RE_INT_EN_W<1> {
        RE_INT_EN_W::new(self)
    }
    #[doc = "Bit 2 - Command Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc_int_en(&mut self) -> CC_INT_EN_W<2> {
        CC_INT_EN_W::new(self)
    }
    #[doc = "Bit 3 - Data Transfer Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtc_int_en(&mut self) -> DTC_INT_EN_W<3> {
        DTC_INT_EN_W::new(self)
    }
    #[doc = "Bit 4 - Data Transmit Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtr_int_en(&mut self) -> DTR_INT_EN_W<4> {
        DTR_INT_EN_W::new(self)
    }
    #[doc = "Bit 5 - Data Receive Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn drr_int_en(&mut self) -> DRR_INT_EN_W<5> {
        DRR_INT_EN_W::new(self)
    }
    #[doc = "Bit 6 - Response CRC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rce_int_en(&mut self) -> RCE_INT_EN_W<6> {
        RCE_INT_EN_W::new(self)
    }
    #[doc = "Bit 7 - Data CRC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dce_int_en(&mut self) -> DCE_INT_EN_W<7> {
        DCE_INT_EN_W::new(self)
    }
    #[doc = "Bit 8 - Response Timeout/Boot ACK Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rto_back_int_en(&mut self) -> RTO_BACK_INT_EN_W<8> {
        RTO_BACK_INT_EN_W::new(self)
    }
    #[doc = "Bit 9 - Data Timeout/Boot Data Start Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dto_bds_int_en(&mut self) -> DTO_BDS_INT_EN_W<9> {
        DTO_BDS_INT_EN_W::new(self)
    }
    #[doc = "Bit 10 - Data Starvation Timeout/V1.8 Switch Done Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsto_vsd_int_en(&mut self) -> DSTO_VSD_INT_EN_W<10> {
        DSTO_VSD_INT_EN_W::new(self)
    }
    #[doc = "Bit 11 - FIFO Underrun/Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fu_fo_int_en(&mut self) -> FU_FO_INT_EN_W<11> {
        FU_FO_INT_EN_W::new(self)
    }
    #[doc = "Bit 12 - Command Busy and Illegal Write Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cb_iw_int_en(&mut self) -> CB_IW_INT_EN_W<12> {
        CB_IW_INT_EN_W::new(self)
    }
    #[doc = "Bit 13 - Data Start Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dse_bc_int_en(&mut self) -> DSE_BC_INT_EN_W<13> {
        DSE_BC_INT_EN_W::new(self)
    }
    #[doc = "Bit 14 - Auto Command Done Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acd_int_en(&mut self) -> ACD_INT_EN_W<14> {
        ACD_INT_EN_W::new(self)
    }
    #[doc = "Bit 15 - Data End-bit Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dee_int_en(&mut self) -> DEE_INT_EN_W<15> {
        DEE_INT_EN_W::new(self)
    }
    #[doc = "Bit 16 - SDIO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_int_en(&mut self) -> SDIO_INT_EN_W<16> {
        SDIO_INT_EN_W::new(self)
    }
    #[doc = "Bit 30 - Card Inserted Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn card_insert_int_en(&mut self) -> CARD_INSERT_INT_EN_W<30> {
        CARD_INSERT_INT_EN_W::new(self)
    }
    #[doc = "Bit 31 - Card Removed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn card_removal_int_en(&mut self) -> CARD_REMOVAL_INT_EN_W<31> {
        CARD_REMOVAL_INT_EN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_intmask to value 0"]
impl crate::Resettable for SMHC_INTMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
