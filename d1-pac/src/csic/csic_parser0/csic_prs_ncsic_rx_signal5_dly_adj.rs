#[doc = "Register `csic_prs_ncsic_rx_signal5_dly_adj` reader"]
pub type R = crate::R<CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC>;
#[doc = "Register `csic_prs_ncsic_rx_signal5_dly_adj` writer"]
pub type W = crate::W<CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC>;
#[doc = "Field `d4_dly` reader - D4_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D4_DLY_R = crate::FieldReader;
#[doc = "Field `d4_dly` writer - D4_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D4_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `d5_dly` reader - D5_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D5_DLY_R = crate::FieldReader;
#[doc = "Field `d5_dly` writer - D5_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D5_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `d6_dly` reader - D6_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D6_DLY_R = crate::FieldReader;
#[doc = "Field `d6_dly` writer - D6_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D6_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `d7_dly` reader - D7_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D7_DLY_R = crate::FieldReader;
#[doc = "Field `d7_dly` writer - D7_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D7_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - D4_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d4_dly(&self) -> D4_DLY_R {
        D4_DLY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - D5_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d5_dly(&self) -> D5_DLY_R {
        D5_DLY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - D6_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d6_dly(&self) -> D6_DLY_R {
        D6_DLY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - D7_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d7_dly(&self) -> D7_DLY_R {
        D7_DLY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - D4_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d4_dly(&mut self) -> D4_DLY_W<CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC> {
        D4_DLY_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - D5_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d5_dly(&mut self) -> D5_DLY_W<CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC> {
        D5_DLY_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - D6_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d6_dly(&mut self) -> D6_DLY_W<CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC> {
        D6_DLY_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - D7_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d7_dly(&mut self) -> D7_DLY_W<CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC> {
        D7_DLY_W::new(self, 24)
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
#[doc = "CSIC Parser NCSIC RX Signal5 Delay Adjust Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_prs_ncsic_rx_signal5_dly_adj::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_prs_ncsic_rx_signal5_dly_adj::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC;
impl crate::RegisterSpec for CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_prs_ncsic_rx_signal5_dly_adj::R`](R) reader structure"]
impl crate::Readable for CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_prs_ncsic_rx_signal5_dly_adj::W`](W) writer structure"]
impl crate::Writable for CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_prs_ncsic_rx_signal5_dly_adj to value 0"]
impl crate::Resettable for CSIC_PRS_NCSIC_RX_SIGNAL5_DLY_ADJ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
