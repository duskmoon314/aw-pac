#[doc = "Register `tv_basic2` reader"]
pub type R = crate::R<TV_BASIC2_SPEC>;
#[doc = "Register `tv_basic2` writer"]
pub type W = crate::W<TV_BASIC2_SPEC>;
#[doc = "Field `tv_yo` reader - Height is TV_YO+1"]
pub type TV_YO_R = crate::FieldReader<u16>;
#[doc = "Field `tv_yo` writer - Height is TV_YO+1"]
pub type TV_YO_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `tv_xo` reader - Width is TV_XO+1"]
pub type TV_XO_R = crate::FieldReader<u16>;
#[doc = "Field `tv_xo` writer - Width is TV_XO+1"]
pub type TV_XO_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Height is TV_YO+1"]
    #[inline(always)]
    pub fn tv_yo(&self) -> TV_YO_R {
        TV_YO_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Width is TV_XO+1"]
    #[inline(always)]
    pub fn tv_xo(&self) -> TV_XO_R {
        TV_XO_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Height is TV_YO+1"]
    #[inline(always)]
    #[must_use]
    pub fn tv_yo(&mut self) -> TV_YO_W<TV_BASIC2_SPEC> {
        TV_YO_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Width is TV_XO+1"]
    #[inline(always)]
    #[must_use]
    pub fn tv_xo(&mut self) -> TV_XO_W<TV_BASIC2_SPEC> {
        TV_XO_W::new(self, 16)
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
#[doc = "TV Basic Timing Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_basic2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_basic2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_BASIC2_SPEC;
impl crate::RegisterSpec for TV_BASIC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_basic2::R`](R) reader structure"]
impl crate::Readable for TV_BASIC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_basic2::W`](W) writer structure"]
impl crate::Writable for TV_BASIC2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_basic2 to value 0"]
impl crate::Resettable for TV_BASIC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
