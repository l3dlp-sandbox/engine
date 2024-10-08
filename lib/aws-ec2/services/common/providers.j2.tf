terraform {
  required_providers {
    aws = {
      source = "hashicorp/aws"
      version    = "5.51.1"
    }
    local = {
      source = "hashicorp/local"
      version = "2.2.3"
    }
    time = {
      source  = "hashicorp/time"
      version = "0.9.0"
    }
  }
  required_version = "1.9.7"
}

provider "aws" {
  region     = "{{ region }}"
  access_key = "{{ aws_access_key }}"
  secret_key = "{{ aws_secret_key }}"
}

resource "time_static" "on_db_create" {}
