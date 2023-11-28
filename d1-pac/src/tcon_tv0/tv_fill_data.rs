#[doc = "Register `tv_fill_data%s` reader"]
pub type R = crate::R<TV_FILL_DATA_SPEC>;
#[doc = "Register `tv_fill_data%s` writer"]
pub type W = crate::W<TV_FILL_DATA_SPEC>;
#[doc = "Field `fill_value` reader - Fill Value"]
pub type FILL_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `fill_value` writer - Fill Value"]
pub type FILL_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Fill Value"]
    #[inline(always)]
    pub fn fill_value(&self) -> FILL_VALUE_R {
        FILL_VALUE_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Fill Value"]
    #[inline(always)]
    #[must_use]
    pub fn fill_value(&mut self) -> FILL_VALUE_W<TV_FILL_DATA_SPEC> {
        FILL_VALUE_W::new(self, 0)
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
#[doc = "TV Fill Data Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_fill_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_fill_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_FILL_DATA_SPEC;
impl crate::RegisterSpec for TV_FILL_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_fill_data::R`](R) reader structure"]
impl crate::Readable for TV_FILL_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_fill_data::W`](W) writer structure"]
impl crate::Writable for TV_FILL_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_fill_data%s to value 0"]
impl crate::Resettable for TV_FILL_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
