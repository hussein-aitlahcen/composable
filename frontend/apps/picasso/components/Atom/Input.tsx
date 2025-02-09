import React from "react";
import {
  Button,
  ButtonProps as MuiButtonProps,
  InputAdornment,
  TextField,
  TextFieldProps,
  Typography,
  useTheme,
  Box,
} from "@mui/material";
import { TokenAsset } from "./TokenAsset";
import { Label, LabelProps as MuiLabelProps } from "./Label";
import { TokenId } from "tokens";
import { BaseAsset } from "./BaseAsset";

export type InputProps = {
  LabelProps?: MuiLabelProps;
  alert?: boolean;
  tokenId?: TokenId;
  icon?: string;
  tokenDescription?: boolean;
  buttonLabel?: string;
  ButtonProps?: MuiButtonProps;
  referenceText?: string;
  setValue?: React.Dispatch<React.SetStateAction<any>>;
  noBorder?: boolean;
} & Omit<TextFieldProps, "label">;

export const Input: React.FC<InputProps> = ({
  LabelProps,
  alert,
  tokenId,
  icon,
  tokenDescription = true,
  buttonLabel,
  ButtonProps,
  referenceText,
  setValue,
  children,
  noBorder,
  InputProps,
  ...rest
}) => {
  const theme = useTheme();
  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setValue && setValue(event.target.value);
  };
  return (
    <Box>
      {LabelProps && <Label {...LabelProps} />}
      <TextField
        fullWidth
        onChange={handleChange}
        InputProps={{
          startAdornment: tokenId ? (
            <InputAdornment position="start">
              <TokenAsset tokenId={tokenId} iconOnly={!tokenDescription} />
            </InputAdornment>
          ) : icon ? (
            <InputAdornment position="start">
              <BaseAsset icon={icon} />
            </InputAdornment>
          ) : undefined,
          endAdornment: buttonLabel ? (
            <Button size="small" disabled={rest.disabled} {...ButtonProps}>
              {buttonLabel}
            </Button>
          ) : (
            referenceText && (
              <Typography
                variant="body2"
                color={rest.disabled ? "secondary.light" : "text.secondary"}
                whiteSpace="nowrap"
              >
                {referenceText}
              </Typography>
            )
          ),
          ...InputProps,
        }}
        {...rest}
      >
        {children}
      </TextField>
    </Box>
  );
};
