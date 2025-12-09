import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any } ? Omit<T, "children"> : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & {
  ref?: U | null;
};

export const validateEnumOrDefault = <T extends string>(
  value: string | null,
  allowedValues: readonly T[],
  defaultValue: T,
): T => {
  return value && allowedValues.includes(value as T) ? (value as T) : defaultValue;
};

const moneyFormatter = new Intl.NumberFormat("pt-PT", {
  style: "currency",
  currency: "EUR",
});

const numberFormatter = new Intl.NumberFormat("pt-PT");

export function formatMoney(value: number | null | undefined) {
  if (value == null) return "—";
  return moneyFormatter.format(value / 100);
}

export function formatNumber(value: number | null | undefined) {
  if (value == null) return "—";
  return numberFormatter.format(value);
}

export const dateToString = (date: Date) =>
  date.toLocaleDateString("pt-PT", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  });

export function formatDate(value: string | null | undefined) {
  if (!value) return "—";
  return dateToString(new Date(value));
}
