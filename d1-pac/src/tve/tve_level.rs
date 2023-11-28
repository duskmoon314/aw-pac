#[doc = "Register `tve_level` reader"]
pub type R = crate::R<TVE_LEVEL_SPEC>;
#[doc = "Register `tve_level` writer"]
pub type W = crate::W<TVE_LEVEL_SPEC>;
#[doc = "Field `black_level` reader - Specify the black level setting. This is 10 bits unsigned integer. Allowed range is from 240 to 1023."]
pub type BLACK_LEVEL_R = crate::FieldReader<u16>;
#[doc = "Field `black_level` writer - Specify the black level setting. This is 10 bits unsigned integer. Allowed range is from 240 to 1023."]
pub type BLACK_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `blank_level` reader - Specify the blank level setting for active lines. This is 10 bits unsigned integer. Allowed range is from 0 to 1023."]
pub type BLANK_LEVEL_R = crate::FieldReader<u16>;
#[doc = "Field `blank_level` writer - Specify the blank level setting for active lines. This is 10 bits unsigned integer. Allowed range is from 0 to 1023."]
pub type BLANK_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Specify the black level setting. This is 10 bits unsigned integer. Allowed range is from 240 to 1023."]
    #[inline(always)]
    pub fn black_level(&self) -> BLACK_LEVEL_R {
        BLACK_LEVEL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Specify the blank level setting for active lines. This is 10 bits unsigned integer. Allowed range is from 0 to 1023."]
    #[inline(always)]
    pub fn blank_level(&self) -> BLANK_LEVEL_R {
        BLANK_LEVEL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Specify the black level setting. This is 10 bits unsigned integer. Allowed range is from 240 to 1023."]
    #[inline(always)]
    #[must_use]
    pub fn black_level(&mut self) -> BLACK_LEVEL_W<TVE_LEVEL_SPEC> {
        BLACK_LEVEL_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Specify the blank level setting for active lines. This is 10 bits unsigned integer. Allowed range is from 0 to 1023."]
    #[inline(always)]
    #[must_use]
    pub fn blank_level(&mut self) -> BLANK_LEVEL_W<TVE_LEVEL_SPEC> {
        BLANK_LEVEL_W::new(self, 16)
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
#[doc = "TV Encoder Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_level::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_level::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_LEVEL_SPEC;
impl crate::RegisterSpec for TVE_LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_level::R`](R) reader structure"]
impl crate::Readable for TVE_LEVEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_level::W`](W) writer structure"]
impl crate::Writable for TVE_LEVEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_level to value 0x00f0_011a"]
impl crate::Resettable for TVE_LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x00f0_011a;
}
