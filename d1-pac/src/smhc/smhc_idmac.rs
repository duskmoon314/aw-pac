#[doc = "Register `smhc_idmac` reader"]
pub type R = crate::R<SMHC_IDMAC_SPEC>;
#[doc = "Register `smhc_idmac` writer"]
pub type W = crate::W<SMHC_IDMAC_SPEC>;
#[doc = "Field `idmac_rst` reader - DMA Reset"]
pub type IDMAC_RST_R = crate::BitReader;
#[doc = "Field `idmac_rst` writer - DMA Reset"]
pub type IDMAC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fix_bust_ctrl` reader - Fixed Burst"]
pub type FIX_BUST_CTRL_R = crate::BitReader;
#[doc = "Field `fix_bust_ctrl` writer - Fixed Burst"]
pub type FIX_BUST_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `idmac_enb` reader - IDMAC Enable"]
pub type IDMAC_ENB_R = crate::BitReader;
#[doc = "Field `idmac_enb` writer - IDMAC Enable"]
pub type IDMAC_ENB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `des_load_ctrl` writer - "]
pub type DES_LOAD_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Reset"]
    #[inline(always)]
    pub fn idmac_rst(&self) -> IDMAC_RST_R {
        IDMAC_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fixed Burst"]
    #[inline(always)]
    pub fn fix_bust_ctrl(&self) -> FIX_BUST_CTRL_R {
        FIX_BUST_CTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - IDMAC Enable"]
    #[inline(always)]
    pub fn idmac_enb(&self) -> IDMAC_ENB_R {
        IDMAC_ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Reset"]
    #[inline(always)]
    #[must_use]
    pub fn idmac_rst(&mut self) -> IDMAC_RST_W<SMHC_IDMAC_SPEC> {
        IDMAC_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Fixed Burst"]
    #[inline(always)]
    #[must_use]
    pub fn fix_bust_ctrl(&mut self) -> FIX_BUST_CTRL_W<SMHC_IDMAC_SPEC> {
        FIX_BUST_CTRL_W::new(self, 1)
    }
    #[doc = "Bit 7 - IDMAC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn idmac_enb(&mut self) -> IDMAC_ENB_W<SMHC_IDMAC_SPEC> {
        IDMAC_ENB_W::new(self, 7)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn des_load_ctrl(&mut self) -> DES_LOAD_CTRL_W<SMHC_IDMAC_SPEC> {
        DES_LOAD_CTRL_W::new(self, 31)
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
#[doc = "IDMAC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_idmac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_idmac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_IDMAC_SPEC;
impl crate::RegisterSpec for SMHC_IDMAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_idmac::R`](R) reader structure"]
impl crate::Readable for SMHC_IDMAC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_idmac::W`](W) writer structure"]
impl crate::Writable for SMHC_IDMAC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_idmac to value 0"]
impl crate::Resettable for SMHC_IDMAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
