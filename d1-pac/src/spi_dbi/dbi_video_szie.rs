#[doc = "Register `dbi_video_szie` reader"]
pub type R = crate::R<DBI_VIDEO_SZIE_SPEC>;
#[doc = "Register `dbi_video_szie` writer"]
pub type W = crate::W<DBI_VIDEO_SZIE_SPEC>;
#[doc = "Field `h_size` reader - "]
pub type H_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `h_size` writer - "]
pub type H_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `v_size` reader - "]
pub type V_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `v_size` writer - "]
pub type V_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn h_size(&self) -> H_SIZE_R {
        H_SIZE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn v_size(&self) -> V_SIZE_R {
        V_SIZE_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn h_size(&mut self) -> H_SIZE_W<DBI_VIDEO_SZIE_SPEC> {
        H_SIZE_W::new(self, 0)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    #[must_use]
    pub fn v_size(&mut self) -> V_SIZE_W<DBI_VIDEO_SZIE_SPEC> {
        V_SIZE_W::new(self, 16)
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
#[doc = "DBI Video Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbi_video_szie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbi_video_szie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBI_VIDEO_SZIE_SPEC;
impl crate::RegisterSpec for DBI_VIDEO_SZIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_video_szie::R`](R) reader structure"]
impl crate::Readable for DBI_VIDEO_SZIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbi_video_szie::W`](W) writer structure"]
impl crate::Writable for DBI_VIDEO_SZIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dbi_video_szie to value 0"]
impl crate::Resettable for DBI_VIDEO_SZIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
