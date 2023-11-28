#[doc = "Register `tv_basic5` reader"]
pub type R = crate::R<TV_BASIC5_SPEC>;
#[doc = "Register `tv_basic5` writer"]
pub type W = crate::W<TV_BASIC5_SPEC>;
#[doc = "Field `v_spw` reader - Vertical Sync Pulse Width (in lines)\n\nTvspw = (VSPW+1) * Th\n\nNote: VT/2 > (VSPW+1)"]
pub type V_SPW_R = crate::FieldReader<u16>;
#[doc = "Field `v_spw` writer - Vertical Sync Pulse Width (in lines)\n\nTvspw = (VSPW+1) * Th\n\nNote: VT/2 > (VSPW+1)"]
pub type V_SPW_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `h_spw` reader - Horizontal Sync Pulse Width (in dclk)\n\nThspw = (HSPW+1) * Tdclk\n\nNote: HT > (HSPW+1)"]
pub type H_SPW_R = crate::FieldReader<u16>;
#[doc = "Field `h_spw` writer - Horizontal Sync Pulse Width (in dclk)\n\nThspw = (HSPW+1) * Tdclk\n\nNote: HT > (HSPW+1)"]
pub type H_SPW_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Vertical Sync Pulse Width (in lines)\n\nTvspw = (VSPW+1) * Th\n\nNote: VT/2 > (VSPW+1)"]
    #[inline(always)]
    pub fn v_spw(&self) -> V_SPW_R {
        V_SPW_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Horizontal Sync Pulse Width (in dclk)\n\nThspw = (HSPW+1) * Tdclk\n\nNote: HT > (HSPW+1)"]
    #[inline(always)]
    pub fn h_spw(&self) -> H_SPW_R {
        H_SPW_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical Sync Pulse Width (in lines)\n\nTvspw = (VSPW+1) * Th\n\nNote: VT/2 > (VSPW+1)"]
    #[inline(always)]
    #[must_use]
    pub fn v_spw(&mut self) -> V_SPW_W<TV_BASIC5_SPEC> {
        V_SPW_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Horizontal Sync Pulse Width (in dclk)\n\nThspw = (HSPW+1) * Tdclk\n\nNote: HT > (HSPW+1)"]
    #[inline(always)]
    #[must_use]
    pub fn h_spw(&mut self) -> H_SPW_W<TV_BASIC5_SPEC> {
        H_SPW_W::new(self, 16)
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
#[doc = "TV Basic Timing Register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_basic5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_basic5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_BASIC5_SPEC;
impl crate::RegisterSpec for TV_BASIC5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_basic5::R`](R) reader structure"]
impl crate::Readable for TV_BASIC5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_basic5::W`](W) writer structure"]
impl crate::Writable for TV_BASIC5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_basic5 to value 0"]
impl crate::Resettable for TV_BASIC5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
