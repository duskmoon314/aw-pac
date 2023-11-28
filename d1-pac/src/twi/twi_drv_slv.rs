#[doc = "Register `twi_drv_slv` reader"]
pub type R = crate::R<TWI_DRV_SLV_SPEC>;
#[doc = "Register `twi_drv_slv` writer"]
pub type W = crate::W<TWI_DRV_SLV_SPEC>;
#[doc = "Field `slv_id_x` reader - SLAX\\[7:0\\]"]
pub type SLV_ID_X_R = crate::FieldReader;
#[doc = "Field `slv_id_x` writer - SLAX\\[7:0\\]"]
pub type SLV_ID_X_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cmd` reader - R/W operation to slave device"]
pub type CMD_R = crate::BitReader<CMD_A>;
#[doc = "R/W operation to slave device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD_A {
    #[doc = "0: `0`"]
    WRITE = 0,
    #[doc = "1: `1`"]
    READ = 1,
}
impl From<CMD_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as u8 != 0
    }
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMD_A {
        match self.bits {
            false => CMD_A::WRITE,
            true => CMD_A::READ,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == CMD_A::WRITE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == CMD_A::READ
    }
}
#[doc = "Field `cmd` writer - R/W operation to slave device"]
pub type CMD_W<'a, REG> = crate::BitWriter<'a, REG, CMD_A>;
impl<'a, REG> CMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::WRITE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::READ)
    }
}
#[doc = "Field `slv_id` reader - Slave device ID"]
pub type SLV_ID_R = crate::FieldReader;
#[doc = "Field `slv_id` writer - Slave device ID"]
pub type SLV_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:7 - SLAX\\[7:0\\]"]
    #[inline(always)]
    pub fn slv_id_x(&self) -> SLV_ID_X_R {
        SLV_ID_X_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - R/W operation to slave device"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Slave device ID"]
    #[inline(always)]
    pub fn slv_id(&self) -> SLV_ID_R {
        SLV_ID_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SLAX\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn slv_id_x(&mut self) -> SLV_ID_X_W<TWI_DRV_SLV_SPEC> {
        SLV_ID_X_W::new(self, 0)
    }
    #[doc = "Bit 8 - R/W operation to slave device"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<TWI_DRV_SLV_SPEC> {
        CMD_W::new(self, 8)
    }
    #[doc = "Bits 9:15 - Slave device ID"]
    #[inline(always)]
    #[must_use]
    pub fn slv_id(&mut self) -> SLV_ID_W<TWI_DRV_SLV_SPEC> {
        SLV_ID_W::new(self, 9)
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
#[doc = "TWI_DRV Slave ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_slv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_slv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_DRV_SLV_SPEC;
impl crate::RegisterSpec for TWI_DRV_SLV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_drv_slv::R`](R) reader structure"]
impl crate::Readable for TWI_DRV_SLV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_drv_slv::W`](W) writer structure"]
impl crate::Writable for TWI_DRV_SLV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_drv_slv to value 0"]
impl crate::Resettable for TWI_DRV_SLV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
