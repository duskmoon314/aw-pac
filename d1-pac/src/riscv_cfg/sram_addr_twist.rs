#[doc = "Register `sram_addr_twist` reader"]
pub type R = crate::R<SRAM_ADDR_TWIST_SPEC>;
#[doc = "Register `sram_addr_twist` writer"]
pub type W = crate::W<SRAM_ADDR_TWIST_SPEC>;
#[doc = "Field `sram_addr_ts_fg` reader - SRAM Address Twist Flag"]
pub type SRAM_ADDR_TS_FG_R = crate::BitReader;
#[doc = "Field `sram_addr_ts_fg` writer - SRAM Address Twist Flag"]
pub type SRAM_ADDR_TS_FG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sram_ts_kf` writer - SRAM Twist Keyfield"]
pub type SRAM_TS_KF_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - SRAM Address Twist Flag"]
    #[inline(always)]
    pub fn sram_addr_ts_fg(&self) -> SRAM_ADDR_TS_FG_R {
        SRAM_ADDR_TS_FG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM Address Twist Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sram_addr_ts_fg(&mut self) -> SRAM_ADDR_TS_FG_W<SRAM_ADDR_TWIST_SPEC> {
        SRAM_ADDR_TS_FG_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRAM Twist Keyfield"]
    #[inline(always)]
    #[must_use]
    pub fn sram_ts_kf(&mut self) -> SRAM_TS_KF_W<SRAM_ADDR_TWIST_SPEC> {
        SRAM_TS_KF_W::new(self, 16)
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
#[doc = "SRAM Address Twist Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_addr_twist::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_addr_twist::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_ADDR_TWIST_SPEC;
impl crate::RegisterSpec for SRAM_ADDR_TWIST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_addr_twist::R`](R) reader structure"]
impl crate::Readable for SRAM_ADDR_TWIST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_addr_twist::W`](W) writer structure"]
impl crate::Writable for SRAM_ADDR_TWIST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sram_addr_twist to value 0"]
impl crate::Resettable for SRAM_ADDR_TWIST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
