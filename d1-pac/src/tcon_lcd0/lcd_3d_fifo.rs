#[doc = "Register `lcd_3d_fifo` reader"]
pub type R = crate::R<LCD_3D_FIFO_SPEC>;
#[doc = "Register `lcd_3d_fifo` writer"]
pub type W = crate::W<LCD_3D_FIFO_SPEC>;
#[doc = "Field `setting` reader - Set the work mode of 3D FIFO"]
pub type SETTING_R = crate::FieldReader<SETTING_A>;
#[doc = "Set the work mode of 3D FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETTING_A {
    #[doc = "0: Bypass"]
    BYPASS = 0,
    #[doc = "1: Used as normal FIFO"]
    NORMAL = 1,
    #[doc = "2: Used as 3D interlace FIFO"]
    INTERLACE_3D = 2,
}
impl From<SETTING_A> for u8 {
    #[inline(always)]
    fn from(variant: SETTING_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SETTING_A {
    type Ux = u8;
}
impl SETTING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SETTING_A> {
        match self.bits {
            0 => Some(SETTING_A::BYPASS),
            1 => Some(SETTING_A::NORMAL),
            2 => Some(SETTING_A::INTERLACE_3D),
            _ => None,
        }
    }
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == SETTING_A::BYPASS
    }
    #[doc = "Used as normal FIFO"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SETTING_A::NORMAL
    }
    #[doc = "Used as 3D interlace FIFO"]
    #[inline(always)]
    pub fn is_interlace_3d(&self) -> bool {
        *self == SETTING_A::INTERLACE_3D
    }
}
#[doc = "Field `setting` writer - Set the work mode of 3D FIFO"]
pub type SETTING_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SETTING_A>;
impl<'a, REG> SETTING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(SETTING_A::BYPASS)
    }
    #[doc = "Used as normal FIFO"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SETTING_A::NORMAL)
    }
    #[doc = "Used as 3D interlace FIFO"]
    #[inline(always)]
    pub fn interlace_3d(self) -> &'a mut crate::W<REG> {
        self.variant(SETTING_A::INTERLACE_3D)
    }
}
#[doc = "Field `half_line_size` reader - The number of data in half line=3D_FIFO_HALF_LINE_SIZE+1, only valid when 3D_FIFO_SETTING is set as 2."]
pub type HALF_LINE_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `half_line_size` writer - The number of data in half line=3D_FIFO_HALF_LINE_SIZE+1, only valid when 3D_FIFO_SETTING is set as 2."]
pub type HALF_LINE_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `bist_en` reader - Enable the 3D fifo bist test function"]
pub type BIST_EN_R = crate::BitReader<BIST_EN_A>;
#[doc = "Enable the 3D fifo bist test function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIST_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<BIST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BIST_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BIST_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BIST_EN_A {
        match self.bits {
            false => BIST_EN_A::DISABLE,
            true => BIST_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BIST_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BIST_EN_A::ENABLE
    }
}
#[doc = "Field `bist_en` writer - Enable the 3D fifo bist test function"]
pub type BIST_EN_W<'a, REG> = crate::BitWriter<'a, REG, BIST_EN_A>;
impl<'a, REG> BIST_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BIST_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BIST_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Set the work mode of 3D FIFO"]
    #[inline(always)]
    pub fn setting(&self) -> SETTING_R {
        SETTING_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:13 - The number of data in half line=3D_FIFO_HALF_LINE_SIZE+1, only valid when 3D_FIFO_SETTING is set as 2."]
    #[inline(always)]
    pub fn half_line_size(&self) -> HALF_LINE_SIZE_R {
        HALF_LINE_SIZE_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable the 3D fifo bist test function"]
    #[inline(always)]
    pub fn bist_en(&self) -> BIST_EN_R {
        BIST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set the work mode of 3D FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn setting(&mut self) -> SETTING_W<LCD_3D_FIFO_SPEC> {
        SETTING_W::new(self, 0)
    }
    #[doc = "Bits 4:13 - The number of data in half line=3D_FIFO_HALF_LINE_SIZE+1, only valid when 3D_FIFO_SETTING is set as 2."]
    #[inline(always)]
    #[must_use]
    pub fn half_line_size(&mut self) -> HALF_LINE_SIZE_W<LCD_3D_FIFO_SPEC> {
        HALF_LINE_SIZE_W::new(self, 4)
    }
    #[doc = "Bit 31 - Enable the 3D fifo bist test function"]
    #[inline(always)]
    #[must_use]
    pub fn bist_en(&mut self) -> BIST_EN_W<LCD_3D_FIFO_SPEC> {
        BIST_EN_W::new(self, 31)
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
#[doc = "LCD 3D FIFO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_3d_fifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_3d_fifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_3D_FIFO_SPEC;
impl crate::RegisterSpec for LCD_3D_FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_3d_fifo::R`](R) reader structure"]
impl crate::Readable for LCD_3D_FIFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_3d_fifo::W`](W) writer structure"]
impl crate::Writable for LCD_3D_FIFO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_3d_fifo to value 0"]
impl crate::Resettable for LCD_3D_FIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
