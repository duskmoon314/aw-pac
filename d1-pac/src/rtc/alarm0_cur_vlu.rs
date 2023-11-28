#[doc = "Register `alarm0_cur_vlu` reader"]
pub type R = crate::R<ALARM0_CUR_VLU_SPEC>;
#[doc = "Register `alarm0_cur_vlu` writer"]
pub type W = crate::W<ALARM0_CUR_VLU_SPEC>;
#[doc = "Field `second` reader - Current second Range from 0 to 59."]
pub type SECOND_R = crate::FieldReader;
#[doc = "Field `second` writer - Current second Range from 0 to 59."]
pub type SECOND_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `minute` reader - Current minute Range from 0 to 59."]
pub type MINUTE_R = crate::FieldReader;
#[doc = "Field `minute` writer - Current minute Range from 0 to 59."]
pub type MINUTE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `hour` reader - Current hour Range from 0 to 23."]
pub type HOUR_R = crate::FieldReader;
#[doc = "Field `hour` writer - Current hour Range from 0 to 23."]
pub type HOUR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - Current second Range from 0 to 59."]
    #[inline(always)]
    pub fn second(&self) -> SECOND_R {
        SECOND_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Current minute Range from 0 to 59."]
    #[inline(always)]
    pub fn minute(&self) -> MINUTE_R {
        MINUTE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Current hour Range from 0 to 23."]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Current second Range from 0 to 59."]
    #[inline(always)]
    #[must_use]
    pub fn second(&mut self) -> SECOND_W<ALARM0_CUR_VLU_SPEC> {
        SECOND_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Current minute Range from 0 to 59."]
    #[inline(always)]
    #[must_use]
    pub fn minute(&mut self) -> MINUTE_W<ALARM0_CUR_VLU_SPEC> {
        MINUTE_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Current hour Range from 0 to 23."]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HOUR_W<ALARM0_CUR_VLU_SPEC> {
        HOUR_W::new(self, 16)
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
#[doc = "Alarm 0 Counter Current Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm0_cur_vlu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0_cur_vlu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARM0_CUR_VLU_SPEC;
impl crate::RegisterSpec for ALARM0_CUR_VLU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm0_cur_vlu::R`](R) reader structure"]
impl crate::Readable for ALARM0_CUR_VLU_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarm0_cur_vlu::W`](W) writer structure"]
impl crate::Writable for ALARM0_CUR_VLU_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets alarm0_cur_vlu to value 0"]
impl crate::Resettable for ALARM0_CUR_VLU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
