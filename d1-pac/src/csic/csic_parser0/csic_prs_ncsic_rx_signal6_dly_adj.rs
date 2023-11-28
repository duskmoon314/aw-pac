#[doc = "Register `csic_prs_ncsic_rx_signal6_dly_adj` reader"]
pub type R = crate::R<CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC>;
#[doc = "Register `csic_prs_ncsic_rx_signal6_dly_adj` writer"]
pub type W = crate::W<CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC>;
#[doc = "Field `d0_dly` reader - D0_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D0_DLY_R = crate::FieldReader;
#[doc = "Field `d0_dly` writer - D0_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D0_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `d1_dly` reader - D1_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D1_DLY_R = crate::FieldReader;
#[doc = "Field `d1_dly` writer - D1_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D1_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `d2_dly` reader - D2_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D2_DLY_R = crate::FieldReader;
#[doc = "Field `d2_dly` writer - D2_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D2_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `d3_dly` reader - D3_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D3_DLY_R = crate::FieldReader;
#[doc = "Field `d3_dly` writer - D3_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type D3_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - D0_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d0_dly(&self) -> D0_DLY_R {
        D0_DLY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - D1_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d1_dly(&self) -> D1_DLY_R {
        D1_DLY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - D2_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d2_dly(&self) -> D2_DLY_R {
        D2_DLY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - D3_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn d3_dly(&self) -> D3_DLY_R {
        D3_DLY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - D0_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d0_dly(&mut self) -> D0_DLY_W<CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC> {
        D0_DLY_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - D1_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d1_dly(&mut self) -> D1_DLY_W<CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC> {
        D1_DLY_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - D2_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d2_dly(&mut self) -> D2_DLY_W<CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC> {
        D2_DLY_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - D3_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn d3_dly(&mut self) -> D3_DLY_W<CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC> {
        D3_DLY_W::new(self, 24)
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
#[doc = "CSIC Parser NCSIC RX Signal6 Delay Adjust Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_prs_ncsic_rx_signal6_dly_adj::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_prs_ncsic_rx_signal6_dly_adj::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC;
impl crate::RegisterSpec for CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_prs_ncsic_rx_signal6_dly_adj::R`](R) reader structure"]
impl crate::Readable for CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_prs_ncsic_rx_signal6_dly_adj::W`](W) writer structure"]
impl crate::Writable for CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_prs_ncsic_rx_signal6_dly_adj to value 0"]
impl crate::Resettable for CSIC_PRS_NCSIC_RX_SIGNAL6_DLY_ADJ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
