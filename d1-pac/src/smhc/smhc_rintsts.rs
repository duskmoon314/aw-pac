#[doc = "Register `smhc_rintsts` reader"]
pub type R = crate::R<SMHC_RINTSTS_SPEC>;
#[doc = "Register `smhc_rintsts` writer"]
pub type W = crate::W<SMHC_RINTSTS_SPEC>;
#[doc = "Field `re` reader - Response Error"]
pub type RE_R = crate::BitReader;
#[doc = "Field `re` writer - Response Error"]
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cc` reader - Command Complete"]
pub type CC_R = crate::BitReader;
#[doc = "Field `cc` writer - Command Complete"]
pub type CC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dtc` reader - Data Transfer Complete"]
pub type DTC_R = crate::BitReader;
#[doc = "Field `dtc` writer - Data Transfer Complete"]
pub type DTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dtr` reader - Data Transmit Request"]
pub type DTR_R = crate::BitReader;
#[doc = "Field `dtr` writer - Data Transmit Request"]
pub type DTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `drr` reader - Data Receive Request"]
pub type DRR_R = crate::BitReader;
#[doc = "Field `drr` writer - Data Receive Request"]
pub type DRR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rce` reader - Response CRC Error"]
pub type RCE_R = crate::BitReader;
#[doc = "Field `rce` writer - Response CRC Error"]
pub type RCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dce` reader - Data CRC Error"]
pub type DCE_R = crate::BitReader;
#[doc = "Field `dce` writer - Data CRC Error"]
pub type DCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rto_back` reader - Response Timeout/Boot ACK Received"]
pub type RTO_BACK_R = crate::BitReader;
#[doc = "Field `rto_back` writer - Response Timeout/Boot ACK Received"]
pub type RTO_BACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dto_bds` reader - Data Timeout/Boot Data Start"]
pub type DTO_BDS_R = crate::BitReader;
#[doc = "Field `dto_bds` writer - Data Timeout/Boot Data Start"]
pub type DTO_BDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dsto_vsd` reader - Data Starvation Timeout/V1.8 Switch Done"]
pub type DSTO_VSD_R = crate::BitReader;
#[doc = "Field `dsto_vsd` writer - Data Starvation Timeout/V1.8 Switch Done"]
pub type DSTO_VSD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fu_fo` reader - FIFO Underrun/Overflow"]
pub type FU_FO_R = crate::BitReader;
#[doc = "Field `fu_fo` writer - FIFO Underrun/Overflow"]
pub type FU_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cb_iw` reader - Command Busy and Illegal Write"]
pub type CB_IW_R = crate::BitReader;
#[doc = "Field `cb_iw` writer - Command Busy and Illegal Write"]
pub type CB_IW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dse_bc` reader - Data Start Error/Busy Clear"]
pub type DSE_BC_R = crate::BitReader;
#[doc = "Field `dse_bc` writer - Data Start Error/Busy Clear"]
pub type DSE_BC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acd` reader - Auto Command Done"]
pub type ACD_R = crate::BitReader;
#[doc = "Field `acd` writer - Auto Command Done"]
pub type ACD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dee` reader - Data End-bit Error"]
pub type DEE_R = crate::BitReader;
#[doc = "Field `dee` writer - Data End-bit Error"]
pub type DEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdioi_int` reader - SDIO Interrupt"]
pub type SDIOI_INT_R = crate::BitReader;
#[doc = "Field `sdioi_int` writer - SDIO Interrupt"]
pub type SDIOI_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `card_insert` reader - Card Inserted"]
pub type CARD_INSERT_R = crate::BitReader;
#[doc = "Field `card_insert` writer - Card Inserted"]
pub type CARD_INSERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `card_removal` reader - Card Removed"]
pub type CARD_REMOVAL_R = crate::BitReader;
#[doc = "Field `card_removal` writer - Card Removed"]
pub type CARD_REMOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Response Error"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command Complete"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Transfer Complete"]
    #[inline(always)]
    pub fn dtc(&self) -> DTC_R {
        DTC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Transmit Request"]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Receive Request"]
    #[inline(always)]
    pub fn drr(&self) -> DRR_R {
        DRR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Response CRC Error"]
    #[inline(always)]
    pub fn rce(&self) -> RCE_R {
        RCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data CRC Error"]
    #[inline(always)]
    pub fn dce(&self) -> DCE_R {
        DCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Response Timeout/Boot ACK Received"]
    #[inline(always)]
    pub fn rto_back(&self) -> RTO_BACK_R {
        RTO_BACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Timeout/Boot Data Start"]
    #[inline(always)]
    pub fn dto_bds(&self) -> DTO_BDS_R {
        DTO_BDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Starvation Timeout/V1.8 Switch Done"]
    #[inline(always)]
    pub fn dsto_vsd(&self) -> DSTO_VSD_R {
        DSTO_VSD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FIFO Underrun/Overflow"]
    #[inline(always)]
    pub fn fu_fo(&self) -> FU_FO_R {
        FU_FO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Command Busy and Illegal Write"]
    #[inline(always)]
    pub fn cb_iw(&self) -> CB_IW_R {
        CB_IW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data Start Error/Busy Clear"]
    #[inline(always)]
    pub fn dse_bc(&self) -> DSE_BC_R {
        DSE_BC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto Command Done"]
    #[inline(always)]
    pub fn acd(&self) -> ACD_R {
        ACD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data End-bit Error"]
    #[inline(always)]
    pub fn dee(&self) -> DEE_R {
        DEE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SDIO Interrupt"]
    #[inline(always)]
    pub fn sdioi_int(&self) -> SDIOI_INT_R {
        SDIOI_INT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 30 - Card Inserted"]
    #[inline(always)]
    pub fn card_insert(&self) -> CARD_INSERT_R {
        CARD_INSERT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Card Removed"]
    #[inline(always)]
    pub fn card_removal(&self) -> CARD_REMOVAL_R {
        CARD_REMOVAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Response Error"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<SMHC_RINTSTS_SPEC> {
        RE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Command Complete"]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CC_W<SMHC_RINTSTS_SPEC> {
        CC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Data Transfer Complete"]
    #[inline(always)]
    #[must_use]
    pub fn dtc(&mut self) -> DTC_W<SMHC_RINTSTS_SPEC> {
        DTC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Data Transmit Request"]
    #[inline(always)]
    #[must_use]
    pub fn dtr(&mut self) -> DTR_W<SMHC_RINTSTS_SPEC> {
        DTR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Data Receive Request"]
    #[inline(always)]
    #[must_use]
    pub fn drr(&mut self) -> DRR_W<SMHC_RINTSTS_SPEC> {
        DRR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Response CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn rce(&mut self) -> RCE_W<SMHC_RINTSTS_SPEC> {
        RCE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Data CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn dce(&mut self) -> DCE_W<SMHC_RINTSTS_SPEC> {
        DCE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Response Timeout/Boot ACK Received"]
    #[inline(always)]
    #[must_use]
    pub fn rto_back(&mut self) -> RTO_BACK_W<SMHC_RINTSTS_SPEC> {
        RTO_BACK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Data Timeout/Boot Data Start"]
    #[inline(always)]
    #[must_use]
    pub fn dto_bds(&mut self) -> DTO_BDS_W<SMHC_RINTSTS_SPEC> {
        DTO_BDS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data Starvation Timeout/V1.8 Switch Done"]
    #[inline(always)]
    #[must_use]
    pub fn dsto_vsd(&mut self) -> DSTO_VSD_W<SMHC_RINTSTS_SPEC> {
        DSTO_VSD_W::new(self, 10)
    }
    #[doc = "Bit 11 - FIFO Underrun/Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn fu_fo(&mut self) -> FU_FO_W<SMHC_RINTSTS_SPEC> {
        FU_FO_W::new(self, 11)
    }
    #[doc = "Bit 12 - Command Busy and Illegal Write"]
    #[inline(always)]
    #[must_use]
    pub fn cb_iw(&mut self) -> CB_IW_W<SMHC_RINTSTS_SPEC> {
        CB_IW_W::new(self, 12)
    }
    #[doc = "Bit 13 - Data Start Error/Busy Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dse_bc(&mut self) -> DSE_BC_W<SMHC_RINTSTS_SPEC> {
        DSE_BC_W::new(self, 13)
    }
    #[doc = "Bit 14 - Auto Command Done"]
    #[inline(always)]
    #[must_use]
    pub fn acd(&mut self) -> ACD_W<SMHC_RINTSTS_SPEC> {
        ACD_W::new(self, 14)
    }
    #[doc = "Bit 15 - Data End-bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn dee(&mut self) -> DEE_W<SMHC_RINTSTS_SPEC> {
        DEE_W::new(self, 15)
    }
    #[doc = "Bit 16 - SDIO Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sdioi_int(&mut self) -> SDIOI_INT_W<SMHC_RINTSTS_SPEC> {
        SDIOI_INT_W::new(self, 16)
    }
    #[doc = "Bit 30 - Card Inserted"]
    #[inline(always)]
    #[must_use]
    pub fn card_insert(&mut self) -> CARD_INSERT_W<SMHC_RINTSTS_SPEC> {
        CARD_INSERT_W::new(self, 30)
    }
    #[doc = "Bit 31 - Card Removed"]
    #[inline(always)]
    #[must_use]
    pub fn card_removal(&mut self) -> CARD_REMOVAL_W<SMHC_RINTSTS_SPEC> {
        CARD_REMOVAL_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_rintsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_rintsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_RINTSTS_SPEC;
impl crate::RegisterSpec for SMHC_RINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_rintsts::R`](R) reader structure"]
impl crate::Readable for SMHC_RINTSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_rintsts::W`](W) writer structure"]
impl crate::Writable for SMHC_RINTSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_rintsts to value 0"]
impl crate::Resettable for SMHC_RINTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
