#[doc = "Register `lcd_safe_period` reader"]
pub type R = crate::R<LCD_SAFE_PERIOD_SPEC>;
#[doc = "Register `lcd_safe_period` writer"]
pub type W = crate::W<LCD_SAFE_PERIOD_SPEC>;
#[doc = "Field `safe_period_mode` reader - Select the save mode"]
pub type SAFE_PERIOD_MODE_R = crate::FieldReader<SAFE_PERIOD_MODE_A>;
#[doc = "Select the save mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAFE_PERIOD_MODE_A {
    #[doc = "0: unsafe"]
    UNSAFE = 0,
    #[doc = "1: safe"]
    SAFE = 1,
    #[doc = "2: safe at FIFO_CURR_NUM > SAFE_PERIOD_FIFO_NUM"]
    SAFE_FIFO_CURR_NUM_GT_SAFE_PERIOD_FIFO_NUM = 2,
    #[doc = "3: safe at 2 and safe at sync active"]
    SAFE_2_SYNC_ACTIVE = 3,
    #[doc = "4: safe at line"]
    SAFE_LINE = 4,
}
impl From<SAFE_PERIOD_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SAFE_PERIOD_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAFE_PERIOD_MODE_A {
    type Ux = u8;
}
impl SAFE_PERIOD_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SAFE_PERIOD_MODE_A> {
        match self.bits {
            0 => Some(SAFE_PERIOD_MODE_A::UNSAFE),
            1 => Some(SAFE_PERIOD_MODE_A::SAFE),
            2 => Some(SAFE_PERIOD_MODE_A::SAFE_FIFO_CURR_NUM_GT_SAFE_PERIOD_FIFO_NUM),
            3 => Some(SAFE_PERIOD_MODE_A::SAFE_2_SYNC_ACTIVE),
            4 => Some(SAFE_PERIOD_MODE_A::SAFE_LINE),
            _ => None,
        }
    }
    #[doc = "unsafe"]
    #[inline(always)]
    pub fn is_unsafe(&self) -> bool {
        *self == SAFE_PERIOD_MODE_A::UNSAFE
    }
    #[doc = "safe"]
    #[inline(always)]
    pub fn is_safe(&self) -> bool {
        *self == SAFE_PERIOD_MODE_A::SAFE
    }
    #[doc = "safe at FIFO_CURR_NUM > SAFE_PERIOD_FIFO_NUM"]
    #[inline(always)]
    pub fn is_safe_fifo_curr_num_gt_safe_period_fifo_num(&self) -> bool {
        *self == SAFE_PERIOD_MODE_A::SAFE_FIFO_CURR_NUM_GT_SAFE_PERIOD_FIFO_NUM
    }
    #[doc = "safe at 2 and safe at sync active"]
    #[inline(always)]
    pub fn is_safe_2_sync_active(&self) -> bool {
        *self == SAFE_PERIOD_MODE_A::SAFE_2_SYNC_ACTIVE
    }
    #[doc = "safe at line"]
    #[inline(always)]
    pub fn is_safe_line(&self) -> bool {
        *self == SAFE_PERIOD_MODE_A::SAFE_LINE
    }
}
#[doc = "Field `safe_period_mode` writer - Select the save mode"]
pub type SAFE_PERIOD_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SAFE_PERIOD_MODE_A>;
impl<'a, REG> SAFE_PERIOD_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "unsafe"]
    #[inline(always)]
    pub fn unsafe_(self) -> &'a mut crate::W<REG> {
        self.variant(SAFE_PERIOD_MODE_A::UNSAFE)
    }
    #[doc = "safe"]
    #[inline(always)]
    pub fn safe(self) -> &'a mut crate::W<REG> {
        self.variant(SAFE_PERIOD_MODE_A::SAFE)
    }
    #[doc = "safe at FIFO_CURR_NUM > SAFE_PERIOD_FIFO_NUM"]
    #[inline(always)]
    pub fn safe_fifo_curr_num_gt_safe_period_fifo_num(self) -> &'a mut crate::W<REG> {
        self.variant(SAFE_PERIOD_MODE_A::SAFE_FIFO_CURR_NUM_GT_SAFE_PERIOD_FIFO_NUM)
    }
    #[doc = "safe at 2 and safe at sync active"]
    #[inline(always)]
    pub fn safe_2_sync_active(self) -> &'a mut crate::W<REG> {
        self.variant(SAFE_PERIOD_MODE_A::SAFE_2_SYNC_ACTIVE)
    }
    #[doc = "safe at line"]
    #[inline(always)]
    pub fn safe_line(self) -> &'a mut crate::W<REG> {
        self.variant(SAFE_PERIOD_MODE_A::SAFE_LINE)
    }
}
#[doc = "Field `safe_period_line` reader - Set a fixed line and during the line time, the LCD controller allow dram controller to change frequency. The fixed line should be set in the blanking area."]
pub type SAFE_PERIOD_LINE_R = crate::FieldReader<u16>;
#[doc = "Field `safe_period_line` writer - Set a fixed line and during the line time, the LCD controller allow dram controller to change frequency. The fixed line should be set in the blanking area."]
pub type SAFE_PERIOD_LINE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `safe_period_fifo_num` reader - When the data length in line buffer is more than SAFE_PERIOD_FIFO_NUM, the LCD controller will allow dram controller to stop working to change frequency."]
pub type SAFE_PERIOD_FIFO_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `safe_period_fifo_num` writer - When the data length in line buffer is more than SAFE_PERIOD_FIFO_NUM, the LCD controller will allow dram controller to stop working to change frequency."]
pub type SAFE_PERIOD_FIFO_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:2 - Select the save mode"]
    #[inline(always)]
    pub fn safe_period_mode(&self) -> SAFE_PERIOD_MODE_R {
        SAFE_PERIOD_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:15 - Set a fixed line and during the line time, the LCD controller allow dram controller to change frequency. The fixed line should be set in the blanking area."]
    #[inline(always)]
    pub fn safe_period_line(&self) -> SAFE_PERIOD_LINE_R {
        SAFE_PERIOD_LINE_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:28 - When the data length in line buffer is more than SAFE_PERIOD_FIFO_NUM, the LCD controller will allow dram controller to stop working to change frequency."]
    #[inline(always)]
    pub fn safe_period_fifo_num(&self) -> SAFE_PERIOD_FIFO_NUM_R {
        SAFE_PERIOD_FIFO_NUM_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select the save mode"]
    #[inline(always)]
    #[must_use]
    pub fn safe_period_mode(&mut self) -> SAFE_PERIOD_MODE_W<LCD_SAFE_PERIOD_SPEC> {
        SAFE_PERIOD_MODE_W::new(self, 0)
    }
    #[doc = "Bits 4:15 - Set a fixed line and during the line time, the LCD controller allow dram controller to change frequency. The fixed line should be set in the blanking area."]
    #[inline(always)]
    #[must_use]
    pub fn safe_period_line(&mut self) -> SAFE_PERIOD_LINE_W<LCD_SAFE_PERIOD_SPEC> {
        SAFE_PERIOD_LINE_W::new(self, 4)
    }
    #[doc = "Bits 16:28 - When the data length in line buffer is more than SAFE_PERIOD_FIFO_NUM, the LCD controller will allow dram controller to stop working to change frequency."]
    #[inline(always)]
    #[must_use]
    pub fn safe_period_fifo_num(&mut self) -> SAFE_PERIOD_FIFO_NUM_W<LCD_SAFE_PERIOD_SPEC> {
        SAFE_PERIOD_FIFO_NUM_W::new(self, 16)
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
#[doc = "LCD Safe Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_safe_period::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_safe_period::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_SAFE_PERIOD_SPEC;
impl crate::RegisterSpec for LCD_SAFE_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_safe_period::R`](R) reader structure"]
impl crate::Readable for LCD_SAFE_PERIOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_safe_period::W`](W) writer structure"]
impl crate::Writable for LCD_SAFE_PERIOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_safe_period to value 0"]
impl crate::Resettable for LCD_SAFE_PERIOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
