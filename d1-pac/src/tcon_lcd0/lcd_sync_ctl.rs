#[doc = "Register `lcd_sync_ctl` reader"]
pub struct R(crate::R<LCD_SYNC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_SYNC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_SYNC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_SYNC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_sync_ctl` writer"]
pub struct W(crate::W<LCD_SYNC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_SYNC_CTL_SPEC>;
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
impl From<crate::W<LCD_SYNC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_SYNC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lcd_ctrl_sync_mode` reader - LCD Controller Sync Mode\n\nNote: Only use in Single DSI mode."]
pub type LCD_CTRL_SYNC_MODE_R = crate::BitReader<LCD_CTRL_SYNC_MODE_A>;
#[doc = "LCD Controller Sync Mode\n\nNote: Only use in Single DSI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_CTRL_SYNC_MODE_A {
    #[doc = "0: Sync in the first time"]
    S_YNC_FIRST = 0,
    #[doc = "1: Sync every frame"]
    S_YNC_EVERY = 1,
}
impl From<LCD_CTRL_SYNC_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_CTRL_SYNC_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_CTRL_SYNC_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_CTRL_SYNC_MODE_A {
        match self.bits {
            false => LCD_CTRL_SYNC_MODE_A::S_YNC_FIRST,
            true => LCD_CTRL_SYNC_MODE_A::S_YNC_EVERY,
        }
    }
    #[doc = "Checks if the value of the field is `S_YNC_FIRST`"]
    #[inline(always)]
    pub fn is_s_ync_first(&self) -> bool {
        *self == LCD_CTRL_SYNC_MODE_A::S_YNC_FIRST
    }
    #[doc = "Checks if the value of the field is `S_YNC_EVERY`"]
    #[inline(always)]
    pub fn is_s_ync_every(&self) -> bool {
        *self == LCD_CTRL_SYNC_MODE_A::S_YNC_EVERY
    }
}
#[doc = "Field `lcd_ctrl_sync_mode` writer - LCD Controller Sync Mode\n\nNote: Only use in Single DSI mode."]
pub type LCD_CTRL_SYNC_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_SYNC_CTL_SPEC, LCD_CTRL_SYNC_MODE_A, O>;
impl<'a, const O: u8> LCD_CTRL_SYNC_MODE_W<'a, O> {
    #[doc = "Sync in the first time"]
    #[inline(always)]
    pub fn s_ync_first(self) -> &'a mut W {
        self.variant(LCD_CTRL_SYNC_MODE_A::S_YNC_FIRST)
    }
    #[doc = "Sync every frame"]
    #[inline(always)]
    pub fn s_ync_every(self) -> &'a mut W {
        self.variant(LCD_CTRL_SYNC_MODE_A::S_YNC_EVERY)
    }
}
#[doc = "Field `lcd_cyrl_sync_master_slave` reader - LCD Controller Sync Master Slave\n\nNote: Only use in Single DSI mode."]
pub type LCD_CYRL_SYNC_MASTER_SLAVE_R = crate::BitReader<LCD_CYRL_SYNC_MASTER_SLAVE_A>;
#[doc = "LCD Controller Sync Master Slave\n\nNote: Only use in Single DSI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_CYRL_SYNC_MASTER_SLAVE_A {
    #[doc = "0: Master"]
    M_ASTER = 0,
    #[doc = "1: Slave"]
    S_LAVE = 1,
}
impl From<LCD_CYRL_SYNC_MASTER_SLAVE_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_CYRL_SYNC_MASTER_SLAVE_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_CYRL_SYNC_MASTER_SLAVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_CYRL_SYNC_MASTER_SLAVE_A {
        match self.bits {
            false => LCD_CYRL_SYNC_MASTER_SLAVE_A::M_ASTER,
            true => LCD_CYRL_SYNC_MASTER_SLAVE_A::S_LAVE,
        }
    }
    #[doc = "Checks if the value of the field is `M_ASTER`"]
    #[inline(always)]
    pub fn is_m_aster(&self) -> bool {
        *self == LCD_CYRL_SYNC_MASTER_SLAVE_A::M_ASTER
    }
    #[doc = "Checks if the value of the field is `S_LAVE`"]
    #[inline(always)]
    pub fn is_s_lave(&self) -> bool {
        *self == LCD_CYRL_SYNC_MASTER_SLAVE_A::S_LAVE
    }
}
#[doc = "Field `lcd_cyrl_sync_master_slave` writer - LCD Controller Sync Master Slave\n\nNote: Only use in Single DSI mode."]
pub type LCD_CYRL_SYNC_MASTER_SLAVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_SYNC_CTL_SPEC, LCD_CYRL_SYNC_MASTER_SLAVE_A, O>;
impl<'a, const O: u8> LCD_CYRL_SYNC_MASTER_SLAVE_W<'a, O> {
    #[doc = "Master"]
    #[inline(always)]
    pub fn m_aster(self) -> &'a mut W {
        self.variant(LCD_CYRL_SYNC_MASTER_SLAVE_A::M_ASTER)
    }
    #[doc = "Slave"]
    #[inline(always)]
    pub fn s_lave(self) -> &'a mut W {
        self.variant(LCD_CYRL_SYNC_MASTER_SLAVE_A::S_LAVE)
    }
}
#[doc = "Field `lcd_ctrl_work_mode` reader - LCD Controller Work mode"]
pub type LCD_CTRL_WORK_MODE_R = crate::BitReader<LCD_CTRL_WORK_MODE_A>;
#[doc = "LCD Controller Work mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_CTRL_WORK_MODE_A {
    #[doc = "0: Single DSI mode"]
    S_INGLE = 0,
    #[doc = "1: Dual DSI mode"]
    D_UAL = 1,
}
impl From<LCD_CTRL_WORK_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_CTRL_WORK_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_CTRL_WORK_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_CTRL_WORK_MODE_A {
        match self.bits {
            false => LCD_CTRL_WORK_MODE_A::S_INGLE,
            true => LCD_CTRL_WORK_MODE_A::D_UAL,
        }
    }
    #[doc = "Checks if the value of the field is `S_INGLE`"]
    #[inline(always)]
    pub fn is_s_ingle(&self) -> bool {
        *self == LCD_CTRL_WORK_MODE_A::S_INGLE
    }
    #[doc = "Checks if the value of the field is `D_UAL`"]
    #[inline(always)]
    pub fn is_d_ual(&self) -> bool {
        *self == LCD_CTRL_WORK_MODE_A::D_UAL
    }
}
#[doc = "Field `lcd_ctrl_work_mode` writer - LCD Controller Work mode"]
pub type LCD_CTRL_WORK_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_SYNC_CTL_SPEC, LCD_CTRL_WORK_MODE_A, O>;
impl<'a, const O: u8> LCD_CTRL_WORK_MODE_W<'a, O> {
    #[doc = "Single DSI mode"]
    #[inline(always)]
    pub fn s_ingle(self) -> &'a mut W {
        self.variant(LCD_CTRL_WORK_MODE_A::S_INGLE)
    }
    #[doc = "Dual DSI mode"]
    #[inline(always)]
    pub fn d_ual(self) -> &'a mut W {
        self.variant(LCD_CTRL_WORK_MODE_A::D_UAL)
    }
}
impl R {
    #[doc = "Bit 0 - LCD Controller Sync Mode\n\nNote: Only use in Single DSI mode."]
    #[inline(always)]
    pub fn lcd_ctrl_sync_mode(&self) -> LCD_CTRL_SYNC_MODE_R {
        LCD_CTRL_SYNC_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - LCD Controller Sync Master Slave\n\nNote: Only use in Single DSI mode."]
    #[inline(always)]
    pub fn lcd_cyrl_sync_master_slave(&self) -> LCD_CYRL_SYNC_MASTER_SLAVE_R {
        LCD_CYRL_SYNC_MASTER_SLAVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD Controller Work mode"]
    #[inline(always)]
    pub fn lcd_ctrl_work_mode(&self) -> LCD_CTRL_WORK_MODE_R {
        LCD_CTRL_WORK_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Controller Sync Mode\n\nNote: Only use in Single DSI mode."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_ctrl_sync_mode(&mut self) -> LCD_CTRL_SYNC_MODE_W<0> {
        LCD_CTRL_SYNC_MODE_W::new(self)
    }
    #[doc = "Bit 4 - LCD Controller Sync Master Slave\n\nNote: Only use in Single DSI mode."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_cyrl_sync_master_slave(&mut self) -> LCD_CYRL_SYNC_MASTER_SLAVE_W<4> {
        LCD_CYRL_SYNC_MASTER_SLAVE_W::new(self)
    }
    #[doc = "Bit 8 - LCD Controller Work mode"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_ctrl_work_mode(&mut self) -> LCD_CTRL_WORK_MODE_W<8> {
        LCD_CTRL_WORK_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Sync Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_sync_ctl](index.html) module"]
pub struct LCD_SYNC_CTL_SPEC;
impl crate::RegisterSpec for LCD_SYNC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_sync_ctl::R](R) reader structure"]
impl crate::Readable for LCD_SYNC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_sync_ctl::W](W) writer structure"]
impl crate::Writable for LCD_SYNC_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_sync_ctl to value 0"]
impl crate::Resettable for LCD_SYNC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
