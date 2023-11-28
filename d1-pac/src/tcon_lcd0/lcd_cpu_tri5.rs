#[doc = "Register `lcd_cpu_tri5` reader"]
pub type R = crate::R<LCD_CPU_TRI5_SPEC>;
#[doc = "Register `lcd_cpu_tri5` writer"]
pub type W = crate::W<LCD_CPU_TRI5_SPEC>;
#[doc = "Field `d23_to_d0_non_first_valid` reader - Valid in Block except first."]
pub type D23_TO_D0_NON_FIRST_VALID_R = crate::FieldReader<u32>;
#[doc = "Field `d23_to_d0_non_first_valid` writer - Valid in Block except first."]
pub type D23_TO_D0_NON_FIRST_VALID_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `a1_non_first_valid` reader - Valid in Block except first."]
pub type A1_NON_FIRST_VALID_R = crate::BitReader;
#[doc = "Field `a1_non_first_valid` writer - Valid in Block except first."]
pub type A1_NON_FIRST_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Valid in Block except first."]
    #[inline(always)]
    pub fn d23_to_d0_non_first_valid(&self) -> D23_TO_D0_NON_FIRST_VALID_R {
        D23_TO_D0_NON_FIRST_VALID_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Valid in Block except first."]
    #[inline(always)]
    pub fn a1_non_first_valid(&self) -> A1_NON_FIRST_VALID_R {
        A1_NON_FIRST_VALID_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Valid in Block except first."]
    #[inline(always)]
    #[must_use]
    pub fn d23_to_d0_non_first_valid(&mut self) -> D23_TO_D0_NON_FIRST_VALID_W<LCD_CPU_TRI5_SPEC> {
        D23_TO_D0_NON_FIRST_VALID_W::new(self, 0)
    }
    #[doc = "Bit 24 - Valid in Block except first."]
    #[inline(always)]
    #[must_use]
    pub fn a1_non_first_valid(&mut self) -> A1_NON_FIRST_VALID_W<LCD_CPU_TRI5_SPEC> {
        A1_NON_FIRST_VALID_W::new(self, 24)
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
#[doc = "LCD CPU Panel Trigger Register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_tri5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_tri5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CPU_TRI5_SPEC;
impl crate::RegisterSpec for LCD_CPU_TRI5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_cpu_tri5::R`](R) reader structure"]
impl crate::Readable for LCD_CPU_TRI5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_cpu_tri5::W`](W) writer structure"]
impl crate::Writable for LCD_CPU_TRI5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cpu_tri5 to value 0"]
impl crate::Resettable for LCD_CPU_TRI5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
