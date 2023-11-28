#[doc = "Register `csic_prs_ncsic_bt656_head_cfg` reader"]
pub type R = crate::R<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC>;
#[doc = "Register `csic_prs_ncsic_bt656_head_cfg` writer"]
pub type W = crate::W<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC>;
#[doc = "Field `ch_id[0-3]` reader - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
pub type CH_ID_R = crate::FieldReader;
#[doc = "Field `ch_id[0-3]` writer - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
pub type CH_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_id` field"]
    #[inline(always)]
    pub fn ch_id(&self, n: u8) -> CH_ID_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_ID_R::new(((self.bits >> (n * 8)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    pub fn ch0_id(&self) -> CH_ID_R {
        CH_ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    pub fn ch1_id(&self) -> CH_ID_R {
        CH_ID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    pub fn ch2_id(&self) -> CH_ID_R {
        CH_ID_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    pub fn ch3_id(&self) -> CH_ID_R {
        CH_ID_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_id` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_id(&mut self, n: u8) -> CH_ID_W<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_ID_W::new(self, n * 8)
    }
    #[doc = "Bits 0:3 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_id(&mut self) -> CH_ID_W<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC> {
        CH_ID_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_id(&mut self) -> CH_ID_W<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC> {
        CH_ID_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_id(&mut self) -> CH_ID_W<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC> {
        CH_ID_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - The low 4-bit of BT656 header for channel \\[i\\]\n\nOnly valid in BT656 multi-channel mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_id(&mut self) -> CH_ID_W<CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC> {
        CH_ID_W::new(self, 24)
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
#[doc = "CSIC Parser NCSIC BT656 Header Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_prs_ncsic_bt656_head_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_prs_ncsic_bt656_head_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC;
impl crate::RegisterSpec for CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_prs_ncsic_bt656_head_cfg::R`](R) reader structure"]
impl crate::Readable for CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_prs_ncsic_bt656_head_cfg::W`](W) writer structure"]
impl crate::Writable for CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_prs_ncsic_bt656_head_cfg to value 0x0302_0100"]
impl crate::Resettable for CSIC_PRS_NCSIC_BT656_HEAD_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0302_0100;
}
