#[doc = "Register `avs_cnt_div` reader"]
pub type R = crate::R<AVS_CNT_DIV_SPEC>;
#[doc = "Register `avs_cnt_div` writer"]
pub type W = crate::W<AVS_CNT_DIV_SPEC>;
#[doc = "Field `avs_cnt_d[0-1]` reader - The divisor factor of AVS"]
pub type AVS_CNT_D_R = crate::FieldReader<u16>;
#[doc = "Field `avs_cnt_d[0-1]` writer - The divisor factor of AVS"]
pub type AVS_CNT_D_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "The divisor factor of AVS\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `avs_cnt0_d` field"]
    #[inline(always)]
    pub fn avs_cnt_d(&self, n: u8) -> AVS_CNT_D_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        AVS_CNT_D_R::new(((self.bits >> (n * 16)) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - The divisor factor of AVS"]
    #[inline(always)]
    pub fn avs_cnt0_d(&self) -> AVS_CNT_D_R {
        AVS_CNT_D_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - The divisor factor of AVS"]
    #[inline(always)]
    pub fn avs_cnt1_d(&self) -> AVS_CNT_D_R {
        AVS_CNT_D_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "The divisor factor of AVS\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `avs_cnt0_d` field"]
    #[inline(always)]
    #[must_use]
    pub fn avs_cnt_d(&mut self, n: u8) -> AVS_CNT_D_W<AVS_CNT_DIV_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        AVS_CNT_D_W::new(self, n * 16)
    }
    #[doc = "Bits 0:11 - The divisor factor of AVS"]
    #[inline(always)]
    #[must_use]
    pub fn avs_cnt0_d(&mut self) -> AVS_CNT_D_W<AVS_CNT_DIV_SPEC> {
        AVS_CNT_D_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - The divisor factor of AVS"]
    #[inline(always)]
    #[must_use]
    pub fn avs_cnt1_d(&mut self) -> AVS_CNT_D_W<AVS_CNT_DIV_SPEC> {
        AVS_CNT_D_W::new(self, 16)
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
#[doc = "AVS Counter Divisor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`avs_cnt_div::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`avs_cnt_div::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AVS_CNT_DIV_SPEC;
impl crate::RegisterSpec for AVS_CNT_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`avs_cnt_div::R`](R) reader structure"]
impl crate::Readable for AVS_CNT_DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`avs_cnt_div::W`](W) writer structure"]
impl crate::Writable for AVS_CNT_DIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets avs_cnt_div to value 0"]
impl crate::Resettable for AVS_CNT_DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
