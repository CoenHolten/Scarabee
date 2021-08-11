-- MySQL Workbench Forward Engineering

SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema 3ways_db
-- -----------------------------------------------------

-- -----------------------------------------------------
-- Schema 3ways_db
-- -----------------------------------------------------
CREATE SCHEMA IF NOT EXISTS `3ways_db` DEFAULT CHARACTER SET utf8 ;
USE `3ways_db` ;

-- -----------------------------------------------------
-- Table `3ways_db`.`users`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `3ways_db`.`users` (
  `name` VARCHAR(45) NOT NULL,
  `password` VARCHAR(45) NOT NULL,
  `email` VARCHAR(45) NOT NULL,
  `phone` VARCHAR(16) NOT NULL DEFAULT '',
  PRIMARY KEY (`name`));


-- -----------------------------------------------------
-- Table `3ways_db`.`commitments`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `3ways_db`.`commitments` (
  `name` VARCHAR(45) NOT NULL,
  `description` TEXT NOT NULL,
  `is_concept` TINYINT NOT NULL,
  PRIMARY KEY (`name`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `3ways_db`.`initiatives`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `3ways_db`.`initiatives` (
  `commitment` VARCHAR(45) NOT NULL,
  `name` VARCHAR(45) NOT NULL,
  `description` TEXT NOT NULL,
  `user` VARCHAR(45) NULL,
  PRIMARY KEY (`commitment`, `name`),
  INDEX `fk_initiatives_commitments_idx` (`commitment` ASC) VISIBLE,
  INDEX `fk_initiatives_users_idx` (`user` ASC) VISIBLE,
  CONSTRAINT `fk_initiatives_commitments`
    FOREIGN KEY (`commitment`)
    REFERENCES `3ways_db`.`commitments` (`name`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_initiatives_users`
    FOREIGN KEY (`user`)
    REFERENCES `3ways_db`.`users` (`name`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `3ways_db`.`initiative_supports`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `3ways_db`.`initiative_supports` (
  `user` VARCHAR(45) NOT NULL,
  `initiative_commitment` VARCHAR(45) NOT NULL,
  `initiative_name` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`user`, `initiative_commitment`, `initiative_name`),
  INDEX `fk_initiative_supports_initiative_idx` (`initiative_commitment` ASC, `initiative_name` ASC) INVISIBLE,
  CONSTRAINT `fk_initiative_supports_user`
    FOREIGN KEY (`user`)
    REFERENCES `3ways_db`.`users` (`name`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_initiative_supports_initiative`
    FOREIGN KEY (`initiative_commitment` , `initiative_name`)
    REFERENCES `3ways_db`.`initiatives` (`commitment` , `name`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `3ways_db`.`commitment_supports`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `3ways_db`.`commitment_supports` (
  `user` VARCHAR(45) NOT NULL,
  `commitment` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`user`, `commitment`),
  INDEX `fk_commitment_supports_commitment_idx` (`commitment` ASC) VISIBLE,
  CONSTRAINT `fk_commitment_supports_user`
    FOREIGN KEY (`user`)
    REFERENCES `3ways_db`.`users` (`name`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_commitment_supports_commitment`
    FOREIGN KEY (`commitment`)
    REFERENCES `3ways_db`.`commitments` (`name`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;
