#[doc = "Register `tv_basic3` reader"]
pub type R = crate::R<TV_BASIC3_SPEC>;
#[doc = "Register `tv_basic3` writer"]
pub type W = crate::W<TV_BASIC3_SPEC>;
#[doc = "Field `h_bp` reader - Horizontal back porch\n\nThbp = (HBP +1) * Thdclk"]
pub type H_BP_R = crate::FieldReader<u16>;
#[doc = "Field `h_bp` writer - Horizontal back porch\n\nThbp = (HBP +1) * Thdclk"]
pub type H_BP_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `h_t` reader - Horizontal total time\n\nThcycle = (HT+1) * Thdclk"]
pub type H_T_R = crate::FieldReader<u16>;
#[doc = "Field `h_t` writer - Horizontal total time\n\nThcycle = (HT+1) * Thdclk"]
pub type H_T_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:11 - Horizontal back porch\n\nThbp = (HBP +1) * Thdclk"]
    #[inline(always)]
    pub fn h_bp(&self) -> H_BP_R {
        H_BP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:28 - Horizontal total time\n\nThcycle = (HT+1) * Thdclk"]
    #[inline(always)]
    pub fn h_t(&self) -> H_T_R {
        H_T_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Horizontal back porch\n\nThbp = (HBP +1) * Thdclk"]
    #[inline(always)]
    #[must_use]
    pub fn h_bp(&mut self) -> H_BP_W<TV_BASIC3_SPEC> {
        H_BP_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Horizontal total time\n\nThcycle = (HT+1) * Thdclk"]
    #[inline(always)]
    #[must_use]
    pub fn h_t(&mut self) -> H_T_W<TV_BASIC3_SPEC> {
        H_T_W::new(self, 16)
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
#[doc = "TV Basic Timing Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_basic3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_basic3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_BASIC3_SPEC;
impl crate::RegisterSpec for TV_BASIC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_basic3::R`](R) reader structure"]
impl crate::Readable for TV_BASIC3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_basic3::W`](W) writer structure"]
impl crate::Writable for TV_BASIC3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_basic3 to value 0"]
impl crate::Resettable for TV_BASIC3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
