#[doc = "Register `lcd_ceu_ctl` reader"]
pub type R = crate::R<LCD_CEU_CTL_SPEC>;
#[doc = "Register `lcd_ceu_ctl` writer"]
pub type W = crate::W<LCD_CEU_CTL_SPEC>;
#[doc = "Field `bt656_f_mask_value` reader - BT656 F Mask Value"]
pub type BT656_F_MASK_VALUE_R = crate::BitReader;
#[doc = "Field `bt656_f_mask_value` writer - BT656 F Mask Value"]
pub type BT656_F_MASK_VALUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bt656_f_mask` reader - BT656 F Mask"]
pub type BT656_F_MASK_R = crate::BitReader<BT656_F_MASK_A>;
#[doc = "BT656 F Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BT656_F_MASK_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<BT656_F_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: BT656_F_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl BT656_F_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BT656_F_MASK_A {
        match self.bits {
            false => BT656_F_MASK_A::DISABLE,
            true => BT656_F_MASK_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BT656_F_MASK_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BT656_F_MASK_A::ENABLE
    }
}
#[doc = "Field `bt656_f_mask` writer - BT656 F Mask"]
pub type BT656_F_MASK_W<'a, REG> = crate::BitWriter<'a, REG, BT656_F_MASK_A>;
impl<'a, REG> BT656_F_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BT656_F_MASK_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BT656_F_MASK_A::ENABLE)
    }
}
#[doc = "Field `ceu_en` reader - Enable CEU function"]
pub type CEU_EN_R = crate::BitReader<CEU_EN_A>;
#[doc = "Enable CEU function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEU_EN_A {
    #[doc = "0: Bypass"]
    BYPASS = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CEU_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CEU_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CEU_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEU_EN_A {
        match self.bits {
            false => CEU_EN_A::BYPASS,
            true => CEU_EN_A::ENABLE,
        }
    }
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == CEU_EN_A::BYPASS
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CEU_EN_A::ENABLE
    }
}
#[doc = "Field `ceu_en` writer - Enable CEU function"]
pub type CEU_EN_W<'a, REG> = crate::BitWriter<'a, REG, CEU_EN_A>;
impl<'a, REG> CEU_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(CEU_EN_A::BYPASS)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CEU_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 29 - BT656 F Mask Value"]
    #[inline(always)]
    pub fn bt656_f_mask_value(&self) -> BT656_F_MASK_VALUE_R {
        BT656_F_MASK_VALUE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - BT656 F Mask"]
    #[inline(always)]
    pub fn bt656_f_mask(&self) -> BT656_F_MASK_R {
        BT656_F_MASK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable CEU function"]
    #[inline(always)]
    pub fn ceu_en(&self) -> CEU_EN_R {
        CEU_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - BT656 F Mask Value"]
    #[inline(always)]
    #[must_use]
    pub fn bt656_f_mask_value(&mut self) -> BT656_F_MASK_VALUE_W<LCD_CEU_CTL_SPEC> {
        BT656_F_MASK_VALUE_W::new(self, 29)
    }
    #[doc = "Bit 30 - BT656 F Mask"]
    #[inline(always)]
    #[must_use]
    pub fn bt656_f_mask(&mut self) -> BT656_F_MASK_W<LCD_CEU_CTL_SPEC> {
        BT656_F_MASK_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable CEU function"]
    #[inline(always)]
    #[must_use]
    pub fn ceu_en(&mut self) -> CEU_EN_W<LCD_CEU_CTL_SPEC> {
        CEU_EN_W::new(self, 31)
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
#[doc = "LCD CEU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ceu_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ceu_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CEU_CTL_SPEC;
impl crate::RegisterSpec for LCD_CEU_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_ceu_ctl::R`](R) reader structure"]
impl crate::Readable for LCD_CEU_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_ceu_ctl::W`](W) writer structure"]
impl crate::Writable for LCD_CEU_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_ceu_ctl to value 0"]
impl crate::Resettable for LCD_CEU_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
