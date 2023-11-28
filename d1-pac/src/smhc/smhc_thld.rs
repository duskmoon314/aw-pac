#[doc = "Register `smhc_thld` reader"]
pub type R = crate::R<SMHC_THLD_SPEC>;
#[doc = "Register `smhc_thld` writer"]
pub type W = crate::W<SMHC_THLD_SPEC>;
#[doc = "Field `card_rd_thld_enb` reader - Card Read Threshold Enable"]
pub type CARD_RD_THLD_ENB_R = crate::BitReader<CARD_RD_THLD_ENB_A>;
#[doc = "Card Read Threshold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARD_RD_THLD_ENB_A {
    #[doc = "0: Card read threshold disabled"]
    DISABLED = 0,
    #[doc = "1: Card read threshold enabled"]
    ENABLED = 1,
}
impl From<CARD_RD_THLD_ENB_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_RD_THLD_ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl CARD_RD_THLD_ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CARD_RD_THLD_ENB_A {
        match self.bits {
            false => CARD_RD_THLD_ENB_A::DISABLED,
            true => CARD_RD_THLD_ENB_A::ENABLED,
        }
    }
    #[doc = "Card read threshold disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CARD_RD_THLD_ENB_A::DISABLED
    }
    #[doc = "Card read threshold enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CARD_RD_THLD_ENB_A::ENABLED
    }
}
#[doc = "Field `card_rd_thld_enb` writer - Card Read Threshold Enable"]
pub type CARD_RD_THLD_ENB_W<'a, REG> = crate::BitWriter<'a, REG, CARD_RD_THLD_ENB_A>;
impl<'a, REG> CARD_RD_THLD_ENB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card read threshold disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_RD_THLD_ENB_A::DISABLED)
    }
    #[doc = "Card read threshold enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_RD_THLD_ENB_A::ENABLED)
    }
}
#[doc = "Field `bcig` reader - Busy Clear Interrupt Generation"]
pub type BCIG_R = crate::BitReader<BCIG_A>;
#[doc = "Busy Clear Interrupt Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCIG_A {
    #[doc = "0: Busy clear interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Busy clear interrupt enabled"]
    ENABLED = 1,
}
impl From<BCIG_A> for bool {
    #[inline(always)]
    fn from(variant: BCIG_A) -> Self {
        variant as u8 != 0
    }
}
impl BCIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BCIG_A {
        match self.bits {
            false => BCIG_A::DISABLED,
            true => BCIG_A::ENABLED,
        }
    }
    #[doc = "Busy clear interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BCIG_A::DISABLED
    }
    #[doc = "Busy clear interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BCIG_A::ENABLED
    }
}
#[doc = "Field `bcig` writer - Busy Clear Interrupt Generation"]
pub type BCIG_W<'a, REG> = crate::BitWriter<'a, REG, BCIG_A>;
impl<'a, REG> BCIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Busy clear interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BCIG_A::DISABLED)
    }
    #[doc = "Busy clear interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BCIG_A::ENABLED)
    }
}
#[doc = "Field `card_wr_thld_enb` reader - Card Read/Write Threshold Enable"]
pub type CARD_WR_THLD_ENB_R = crate::BitReader<CARD_WR_THLD_ENB_A>;
#[doc = "Card Read/Write Threshold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARD_WR_THLD_ENB_A {
    #[doc = "0: Card write threshold disabled"]
    DISABLED = 0,
    #[doc = "1: Card write threshold enabled"]
    ENABLED = 1,
}
impl From<CARD_WR_THLD_ENB_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_WR_THLD_ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl CARD_WR_THLD_ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CARD_WR_THLD_ENB_A {
        match self.bits {
            false => CARD_WR_THLD_ENB_A::DISABLED,
            true => CARD_WR_THLD_ENB_A::ENABLED,
        }
    }
    #[doc = "Card write threshold disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CARD_WR_THLD_ENB_A::DISABLED
    }
    #[doc = "Card write threshold enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CARD_WR_THLD_ENB_A::ENABLED
    }
}
#[doc = "Field `card_wr_thld_enb` writer - Card Read/Write Threshold Enable"]
pub type CARD_WR_THLD_ENB_W<'a, REG> = crate::BitWriter<'a, REG, CARD_WR_THLD_ENB_A>;
impl<'a, REG> CARD_WR_THLD_ENB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card write threshold disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_WR_THLD_ENB_A::DISABLED)
    }
    #[doc = "Card write threshold enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_WR_THLD_ENB_A::ENABLED)
    }
}
#[doc = "Field `card_wr_thld` reader - Card Read/Write Threshold Size"]
pub type CARD_WR_THLD_R = crate::FieldReader<u16>;
#[doc = "Field `card_wr_thld` writer - Card Read/Write Threshold Size"]
pub type CARD_WR_THLD_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Card Read Threshold Enable"]
    #[inline(always)]
    pub fn card_rd_thld_enb(&self) -> CARD_RD_THLD_ENB_R {
        CARD_RD_THLD_ENB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy Clear Interrupt Generation"]
    #[inline(always)]
    pub fn bcig(&self) -> BCIG_R {
        BCIG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Card Read/Write Threshold Enable"]
    #[inline(always)]
    pub fn card_wr_thld_enb(&self) -> CARD_WR_THLD_ENB_R {
        CARD_WR_THLD_ENB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Card Read/Write Threshold Size"]
    #[inline(always)]
    pub fn card_wr_thld(&self) -> CARD_WR_THLD_R {
        CARD_WR_THLD_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Card Read Threshold Enable"]
    #[inline(always)]
    #[must_use]
    pub fn card_rd_thld_enb(&mut self) -> CARD_RD_THLD_ENB_W<SMHC_THLD_SPEC> {
        CARD_RD_THLD_ENB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Busy Clear Interrupt Generation"]
    #[inline(always)]
    #[must_use]
    pub fn bcig(&mut self) -> BCIG_W<SMHC_THLD_SPEC> {
        BCIG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Card Read/Write Threshold Enable"]
    #[inline(always)]
    #[must_use]
    pub fn card_wr_thld_enb(&mut self) -> CARD_WR_THLD_ENB_W<SMHC_THLD_SPEC> {
        CARD_WR_THLD_ENB_W::new(self, 2)
    }
    #[doc = "Bits 16:27 - Card Read/Write Threshold Size"]
    #[inline(always)]
    #[must_use]
    pub fn card_wr_thld(&mut self) -> CARD_WR_THLD_W<SMHC_THLD_SPEC> {
        CARD_WR_THLD_W::new(self, 16)
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
#[doc = "Card Threshold Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_thld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_thld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_THLD_SPEC;
impl crate::RegisterSpec for SMHC_THLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_thld::R`](R) reader structure"]
impl crate::Readable for SMHC_THLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_thld::W`](W) writer structure"]
impl crate::Writable for SMHC_THLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_thld to value 0"]
impl crate::Resettable for SMHC_THLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
