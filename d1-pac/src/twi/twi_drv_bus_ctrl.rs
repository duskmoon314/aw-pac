#[doc = "Register `twi_drv_bus_ctrl` reader"]
pub type R = crate::R<TWI_DRV_BUS_CTRL_SPEC>;
#[doc = "Register `twi_drv_bus_ctrl` writer"]
pub type W = crate::W<TWI_DRV_BUS_CTRL_SPEC>;
#[doc = "Field `sda_moe` reader - SDA manual output enable"]
pub type SDA_MOE_R = crate::BitReader;
#[doc = "Field `sda_moe` writer - SDA manual output enable"]
pub type SDA_MOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scl_moe` reader - SCL manual output enable"]
pub type SCL_MOE_R = crate::BitReader;
#[doc = "Field `scl_moe` writer - SCL manual output enable"]
pub type SCL_MOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sda_mov` reader - SDA manual output value"]
pub type SDA_MOV_R = crate::BitReader;
#[doc = "Field `sda_mov` writer - SDA manual output value"]
pub type SDA_MOV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scl_mov` reader - SCL manual output value"]
pub type SCL_MOV_R = crate::BitReader;
#[doc = "Field `scl_mov` writer - SCL manual output value"]
pub type SCL_MOV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sda_sta` reader - SDA current status"]
pub type SDA_STA_R = crate::BitReader;
#[doc = "Field `scl_sta` reader - SCL current status"]
pub type SCL_STA_R = crate::BitReader;
#[doc = "Field `clk_m` reader - "]
pub type CLK_M_R = crate::FieldReader;
#[doc = "Field `clk_m` writer - "]
pub type CLK_M_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `clk_n` reader - "]
pub type CLK_N_R = crate::FieldReader;
#[doc = "Field `clk_n` writer - "]
pub type CLK_N_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `clk_duty` reader - Setting duty cycle of clock as master"]
pub type CLK_DUTY_R = crate::BitReader<CLK_DUTY_A>;
#[doc = "Setting duty cycle of clock as master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_DUTY_A {
    #[doc = "0: 50%"]
    P50 = 0,
    #[doc = "1: 40%"]
    P40 = 1,
}
impl From<CLK_DUTY_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_DUTY_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_DUTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_DUTY_A {
        match self.bits {
            false => CLK_DUTY_A::P50,
            true => CLK_DUTY_A::P40,
        }
    }
    #[doc = "50%"]
    #[inline(always)]
    pub fn is_p50(&self) -> bool {
        *self == CLK_DUTY_A::P50
    }
    #[doc = "40%"]
    #[inline(always)]
    pub fn is_p40(&self) -> bool {
        *self == CLK_DUTY_A::P40
    }
}
#[doc = "Field `clk_duty` writer - Setting duty cycle of clock as master"]
pub type CLK_DUTY_W<'a, REG> = crate::BitWriter<'a, REG, CLK_DUTY_A>;
impl<'a, REG> CLK_DUTY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "50%"]
    #[inline(always)]
    pub fn p50(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_DUTY_A::P50)
    }
    #[doc = "40%"]
    #[inline(always)]
    pub fn p40(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_DUTY_A::P40)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_COUNT_MODE_AW {
    #[doc = "0: scl clock high period count on oscl"]
    OSCL = 0,
    #[doc = "1: scl clock high period count on iscl"]
    ISCL = 1,
}
impl From<CLK_COUNT_MODE_AW> for bool {
    #[inline(always)]
    fn from(variant: CLK_COUNT_MODE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `clk_count_mode` writer - "]
pub type CLK_COUNT_MODE_W<'a, REG> = crate::BitWriter<'a, REG, CLK_COUNT_MODE_AW>;
impl<'a, REG> CLK_COUNT_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "scl clock high period count on oscl"]
    #[inline(always)]
    pub fn oscl(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_COUNT_MODE_AW::OSCL)
    }
    #[doc = "scl clock high period count on iscl"]
    #[inline(always)]
    pub fn iscl(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_COUNT_MODE_AW::ISCL)
    }
}
impl R {
    #[doc = "Bit 0 - SDA manual output enable"]
    #[inline(always)]
    pub fn sda_moe(&self) -> SDA_MOE_R {
        SDA_MOE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL manual output enable"]
    #[inline(always)]
    pub fn scl_moe(&self) -> SCL_MOE_R {
        SCL_MOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDA manual output value"]
    #[inline(always)]
    pub fn sda_mov(&self) -> SDA_MOV_R {
        SDA_MOV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCL manual output value"]
    #[inline(always)]
    pub fn scl_mov(&self) -> SCL_MOV_R {
        SCL_MOV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - SDA current status"]
    #[inline(always)]
    pub fn sda_sta(&self) -> SDA_STA_R {
        SDA_STA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCL current status"]
    #[inline(always)]
    pub fn scl_sta(&self) -> SCL_STA_R {
        SCL_STA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn clk_m(&self) -> CLK_M_R {
        CLK_M_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn clk_n(&self) -> CLK_N_R {
        CLK_N_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Setting duty cycle of clock as master"]
    #[inline(always)]
    pub fn clk_duty(&self) -> CLK_DUTY_R {
        CLK_DUTY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDA manual output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sda_moe(&mut self) -> SDA_MOE_W<TWI_DRV_BUS_CTRL_SPEC> {
        SDA_MOE_W::new(self, 0)
    }
    #[doc = "Bit 1 - SCL manual output enable"]
    #[inline(always)]
    #[must_use]
    pub fn scl_moe(&mut self) -> SCL_MOE_W<TWI_DRV_BUS_CTRL_SPEC> {
        SCL_MOE_W::new(self, 1)
    }
    #[doc = "Bit 2 - SDA manual output value"]
    #[inline(always)]
    #[must_use]
    pub fn sda_mov(&mut self) -> SDA_MOV_W<TWI_DRV_BUS_CTRL_SPEC> {
        SDA_MOV_W::new(self, 2)
    }
    #[doc = "Bit 3 - SCL manual output value"]
    #[inline(always)]
    #[must_use]
    pub fn scl_mov(&mut self) -> SCL_MOV_W<TWI_DRV_BUS_CTRL_SPEC> {
        SCL_MOV_W::new(self, 3)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn clk_m(&mut self) -> CLK_M_W<TWI_DRV_BUS_CTRL_SPEC> {
        CLK_M_W::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn clk_n(&mut self) -> CLK_N_W<TWI_DRV_BUS_CTRL_SPEC> {
        CLK_N_W::new(self, 12)
    }
    #[doc = "Bit 15 - Setting duty cycle of clock as master"]
    #[inline(always)]
    #[must_use]
    pub fn clk_duty(&mut self) -> CLK_DUTY_W<TWI_DRV_BUS_CTRL_SPEC> {
        CLK_DUTY_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn clk_count_mode(&mut self) -> CLK_COUNT_MODE_W<TWI_DRV_BUS_CTRL_SPEC> {
        CLK_COUNT_MODE_W::new(self, 16)
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
#[doc = "TWI_DRV Bus Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_bus_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_bus_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_DRV_BUS_CTRL_SPEC;
impl crate::RegisterSpec for TWI_DRV_BUS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_drv_bus_ctrl::R`](R) reader structure"]
impl crate::Readable for TWI_DRV_BUS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_drv_bus_ctrl::W`](W) writer structure"]
impl crate::Writable for TWI_DRV_BUS_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_drv_bus_ctrl to value 0"]
impl crate::Resettable for TWI_DRV_BUS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
