#[doc = "Register `lcd_cpu_tri5` reader"]
pub struct R(crate::R<LCD_CPU_TRI5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CPU_TRI5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CPU_TRI5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CPU_TRI5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_cpu_tri5` writer"]
pub struct W(crate::W<LCD_CPU_TRI5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CPU_TRI5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LCD_CPU_TRI5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CPU_TRI5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `d23_to_d0_non_first_valid` reader - Valid in Block except first."]
pub type D23_TO_D0_NON_FIRST_VALID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `d23_to_d0_non_first_valid` writer - Valid in Block except first."]
pub type D23_TO_D0_NON_FIRST_VALID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CPU_TRI5_SPEC, u32, u32, 24, O>;
#[doc = "Field `a1_non_first_valid` reader - Valid in Block except first."]
pub type A1_NON_FIRST_VALID_R = crate::BitReader<bool>;
#[doc = "Field `a1_non_first_valid` writer - Valid in Block except first."]
pub type A1_NON_FIRST_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_CPU_TRI5_SPEC, bool, O>;
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
    pub fn d23_to_d0_non_first_valid(&mut self) -> D23_TO_D0_NON_FIRST_VALID_W<0> {
        D23_TO_D0_NON_FIRST_VALID_W::new(self)
    }
    #[doc = "Bit 24 - Valid in Block except first."]
    #[inline(always)]
    #[must_use]
    pub fn a1_non_first_valid(&mut self) -> A1_NON_FIRST_VALID_W<24> {
        A1_NON_FIRST_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD CPU Panel Trigger Register5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_cpu_tri5](index.html) module"]
pub struct LCD_CPU_TRI5_SPEC;
impl crate::RegisterSpec for LCD_CPU_TRI5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_cpu_tri5::R](R) reader structure"]
impl crate::Readable for LCD_CPU_TRI5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_cpu_tri5::W](W) writer structure"]
impl crate::Writable for LCD_CPU_TRI5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cpu_tri5 to value 0"]
impl crate::Resettable for LCD_CPU_TRI5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
