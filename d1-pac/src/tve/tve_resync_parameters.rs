#[doc = "Register `tve_resync_parameters` reader"]
pub type R = crate::R<TVE_RESYNC_PARAMETERS_SPEC>;
#[doc = "Register `tve_resync_parameters` writer"]
pub type W = crate::W<TVE_RESYNC_PARAMETERS_SPEC>;
#[doc = "Field `re_sync_pixel_num` reader - Re-sync line pixel from TCON"]
pub type RE_SYNC_PIXEL_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `re_sync_pixel_num` writer - Re-sync line pixel from TCON"]
pub type RE_SYNC_PIXEL_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `re_sync_line_num` reader - Re-sync line number from TCON"]
pub type RE_SYNC_LINE_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `re_sync_line_num` writer - Re-sync line number from TCON"]
pub type RE_SYNC_LINE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `re_sync_dis` reader - "]
pub type RE_SYNC_DIS_R = crate::BitReader<RE_SYNC_DIS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_SYNC_DIS_A {
    #[doc = "0: Re-Sync Enable"]
    ENABLE = 0,
    #[doc = "1: Re-Sync Disable"]
    DISABLE = 1,
}
impl From<RE_SYNC_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: RE_SYNC_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl RE_SYNC_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RE_SYNC_DIS_A {
        match self.bits {
            false => RE_SYNC_DIS_A::ENABLE,
            true => RE_SYNC_DIS_A::DISABLE,
        }
    }
    #[doc = "Re-Sync Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RE_SYNC_DIS_A::ENABLE
    }
    #[doc = "Re-Sync Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RE_SYNC_DIS_A::DISABLE
    }
}
#[doc = "Field `re_sync_dis` writer - "]
pub type RE_SYNC_DIS_W<'a, REG> = crate::BitWriter<'a, REG, RE_SYNC_DIS_A>;
impl<'a, REG> RE_SYNC_DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Re-Sync Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RE_SYNC_DIS_A::ENABLE)
    }
    #[doc = "Re-Sync Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RE_SYNC_DIS_A::DISABLE)
    }
}
#[doc = "Field `re_sync_field` reader - Re-sync field"]
pub type RE_SYNC_FIELD_R = crate::BitReader;
#[doc = "Field `re_sync_field` writer - Re-sync field"]
pub type RE_SYNC_FIELD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Re-sync line pixel from TCON"]
    #[inline(always)]
    pub fn re_sync_pixel_num(&self) -> RE_SYNC_PIXEL_NUM_R {
        RE_SYNC_PIXEL_NUM_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Re-sync line number from TCON"]
    #[inline(always)]
    pub fn re_sync_line_num(&self) -> RE_SYNC_LINE_NUM_R {
        RE_SYNC_LINE_NUM_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn re_sync_dis(&self) -> RE_SYNC_DIS_R {
        RE_SYNC_DIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Re-sync field"]
    #[inline(always)]
    pub fn re_sync_field(&self) -> RE_SYNC_FIELD_R {
        RE_SYNC_FIELD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Re-sync line pixel from TCON"]
    #[inline(always)]
    #[must_use]
    pub fn re_sync_pixel_num(&mut self) -> RE_SYNC_PIXEL_NUM_W<TVE_RESYNC_PARAMETERS_SPEC> {
        RE_SYNC_PIXEL_NUM_W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Re-sync line number from TCON"]
    #[inline(always)]
    #[must_use]
    pub fn re_sync_line_num(&mut self) -> RE_SYNC_LINE_NUM_W<TVE_RESYNC_PARAMETERS_SPEC> {
        RE_SYNC_LINE_NUM_W::new(self, 16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn re_sync_dis(&mut self) -> RE_SYNC_DIS_W<TVE_RESYNC_PARAMETERS_SPEC> {
        RE_SYNC_DIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Re-sync field"]
    #[inline(always)]
    #[must_use]
    pub fn re_sync_field(&mut self) -> RE_SYNC_FIELD_W<TVE_RESYNC_PARAMETERS_SPEC> {
        RE_SYNC_FIELD_W::new(self, 31)
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
#[doc = "TV Encoder Re-sync Parameters Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_resync_parameters::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_resync_parameters::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_RESYNC_PARAMETERS_SPEC;
impl crate::RegisterSpec for TVE_RESYNC_PARAMETERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_resync_parameters::R`](R) reader structure"]
impl crate::Readable for TVE_RESYNC_PARAMETERS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_resync_parameters::W`](W) writer structure"]
impl crate::Writable for TVE_RESYNC_PARAMETERS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_resync_parameters to value 0x0010_0001"]
impl crate::Resettable for TVE_RESYNC_PARAMETERS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0001;
}
