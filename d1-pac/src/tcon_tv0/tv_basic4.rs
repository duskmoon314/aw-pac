#[doc = "Register `tv_basic4` reader"]
pub type R = crate::R<TV_BASIC4_SPEC>;
#[doc = "Register `tv_basic4` writer"]
pub type W = crate::W<TV_BASIC4_SPEC>;
#[doc = "Field `v_bp` reader - horizontal back porch (in HD line)\n\nTvbp = (VBP +1) * Th"]
pub type V_BP_R = crate::FieldReader<u16>;
#[doc = "Field `v_bp` writer - horizontal back porch (in HD line)\n\nTvbp = (VBP +1) * Th"]
pub type V_BP_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `v_t` reader - horizontal total time (in HD line)\n\nTvt = VT/2 * Th"]
pub type V_T_R = crate::FieldReader<u16>;
#[doc = "Field `v_t` writer - horizontal total time (in HD line)\n\nTvt = VT/2 * Th"]
pub type V_T_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:11 - horizontal back porch (in HD line)\n\nTvbp = (VBP +1) * Th"]
    #[inline(always)]
    pub fn v_bp(&self) -> V_BP_R {
        V_BP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:28 - horizontal total time (in HD line)\n\nTvt = VT/2 * Th"]
    #[inline(always)]
    pub fn v_t(&self) -> V_T_R {
        V_T_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - horizontal back porch (in HD line)\n\nTvbp = (VBP +1) * Th"]
    #[inline(always)]
    #[must_use]
    pub fn v_bp(&mut self) -> V_BP_W<TV_BASIC4_SPEC> {
        V_BP_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - horizontal total time (in HD line)\n\nTvt = VT/2 * Th"]
    #[inline(always)]
    #[must_use]
    pub fn v_t(&mut self) -> V_T_W<TV_BASIC4_SPEC> {
        V_T_W::new(self, 16)
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
#[doc = "TV Basic Timing Register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_basic4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_basic4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_BASIC4_SPEC;
impl crate::RegisterSpec for TV_BASIC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_basic4::R`](R) reader structure"]
impl crate::Readable for TV_BASIC4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_basic4::W`](W) writer structure"]
impl crate::Writable for TV_BASIC4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_basic4 to value 0"]
impl crate::Resettable for TV_BASIC4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
