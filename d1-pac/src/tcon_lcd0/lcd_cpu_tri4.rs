#[doc = "Register `lcd_cpu_tri4` reader"]
pub type R = crate::R<LCD_CPU_TRI4_SPEC>;
#[doc = "Register `lcd_cpu_tri4` writer"]
pub type W = crate::W<LCD_CPU_TRI4_SPEC>;
#[doc = "Field `d23_to_d0_first_valid` reader - Valid in first Block."]
pub type D23_TO_D0_FIRST_VALID_R = crate::FieldReader<u32>;
#[doc = "Field `d23_to_d0_first_valid` writer - Valid in first Block."]
pub type D23_TO_D0_FIRST_VALID_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `a1_first_valid` reader - Valid in first Block."]
pub type A1_FIRST_VALID_R = crate::BitReader;
#[doc = "Field `a1_first_valid` writer - Valid in first Block."]
pub type A1_FIRST_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `plug_mode_en` reader - Enable the plug mode used in dsi command mode."]
pub type PLUG_MODE_EN_R = crate::BitReader<PLUG_MODE_EN_A>;
#[doc = "Enable the plug mode used in dsi command mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLUG_MODE_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<PLUG_MODE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLUG_MODE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLUG_MODE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLUG_MODE_EN_A {
        match self.bits {
            false => PLUG_MODE_EN_A::DISABLE,
            true => PLUG_MODE_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLUG_MODE_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLUG_MODE_EN_A::ENABLE
    }
}
#[doc = "Field `plug_mode_en` writer - Enable the plug mode used in dsi command mode."]
pub type PLUG_MODE_EN_W<'a, REG> = crate::BitWriter<'a, REG, PLUG_MODE_EN_A>;
impl<'a, REG> PLUG_MODE_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PLUG_MODE_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PLUG_MODE_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:23 - Valid in first Block."]
    #[inline(always)]
    pub fn d23_to_d0_first_valid(&self) -> D23_TO_D0_FIRST_VALID_R {
        D23_TO_D0_FIRST_VALID_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Valid in first Block."]
    #[inline(always)]
    pub fn a1_first_valid(&self) -> A1_FIRST_VALID_R {
        A1_FIRST_VALID_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable the plug mode used in dsi command mode."]
    #[inline(always)]
    pub fn plug_mode_en(&self) -> PLUG_MODE_EN_R {
        PLUG_MODE_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Valid in first Block."]
    #[inline(always)]
    #[must_use]
    pub fn d23_to_d0_first_valid(&mut self) -> D23_TO_D0_FIRST_VALID_W<LCD_CPU_TRI4_SPEC> {
        D23_TO_D0_FIRST_VALID_W::new(self, 0)
    }
    #[doc = "Bit 24 - Valid in first Block."]
    #[inline(always)]
    #[must_use]
    pub fn a1_first_valid(&mut self) -> A1_FIRST_VALID_W<LCD_CPU_TRI4_SPEC> {
        A1_FIRST_VALID_W::new(self, 24)
    }
    #[doc = "Bit 28 - Enable the plug mode used in dsi command mode."]
    #[inline(always)]
    #[must_use]
    pub fn plug_mode_en(&mut self) -> PLUG_MODE_EN_W<LCD_CPU_TRI4_SPEC> {
        PLUG_MODE_EN_W::new(self, 28)
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
#[doc = "LCD CPU Panel Trigger Register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_tri4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_tri4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CPU_TRI4_SPEC;
impl crate::RegisterSpec for LCD_CPU_TRI4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_cpu_tri4::R`](R) reader structure"]
impl crate::Readable for LCD_CPU_TRI4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_cpu_tri4::W`](W) writer structure"]
impl crate::Writable for LCD_CPU_TRI4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cpu_tri4 to value 0"]
impl crate::Resettable for LCD_CPU_TRI4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
