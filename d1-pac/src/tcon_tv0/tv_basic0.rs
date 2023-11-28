#[doc = "Register `tv_basic0` reader"]
pub type R = crate::R<TV_BASIC0_SPEC>;
#[doc = "Register `tv_basic0` writer"]
pub type W = crate::W<TV_BASIC0_SPEC>;
#[doc = "Field `height_yi` reader - Source Height Is Y+1"]
pub type HEIGHT_YI_R = crate::FieldReader<u16>;
#[doc = "Field `height_yi` writer - Source Height Is Y+1"]
pub type HEIGHT_YI_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `width_xi` reader - Source Width Is X+1"]
pub type WIDTH_XI_R = crate::FieldReader<u16>;
#[doc = "Field `width_xi` writer - Source Width Is X+1"]
pub type WIDTH_XI_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Source Height Is Y+1"]
    #[inline(always)]
    pub fn height_yi(&self) -> HEIGHT_YI_R {
        HEIGHT_YI_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Source Width Is X+1"]
    #[inline(always)]
    pub fn width_xi(&self) -> WIDTH_XI_R {
        WIDTH_XI_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Source Height Is Y+1"]
    #[inline(always)]
    #[must_use]
    pub fn height_yi(&mut self) -> HEIGHT_YI_W<TV_BASIC0_SPEC> {
        HEIGHT_YI_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Source Width Is X+1"]
    #[inline(always)]
    #[must_use]
    pub fn width_xi(&mut self) -> WIDTH_XI_W<TV_BASIC0_SPEC> {
        WIDTH_XI_W::new(self, 16)
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
#[doc = "TV Basic Timing Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_basic0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_basic0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_BASIC0_SPEC;
impl crate::RegisterSpec for TV_BASIC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_basic0::R`](R) reader structure"]
impl crate::Readable for TV_BASIC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_basic0::W`](W) writer structure"]
impl crate::Writable for TV_BASIC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_basic0 to value 0"]
impl crate::Resettable for TV_BASIC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
