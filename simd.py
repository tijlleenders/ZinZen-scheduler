# ---------------------------------------------------------------------------------------------
# Chrono lib providers (https://github.com/chronotope/chrono)
# ---------------------------------------------------------------------------------------------

import datetime
import math


def div_mod_floor(a, b):
    return math.floor(a / b), math.floor(a % b)


def ChronoNaiveDateProvider(valobj, _dict):
    # type: (SBValue, dict) -> str
    ymdf = valobj.GetChildMemberWithName("ymdf").GetValueAsSigned()
    year = ymdf >> 13
    day_of_year = (ymdf & 8191) >> 4
    date = datetime.date(year - 1, 12, 31) + datetime.timedelta(days=day_of_year)
    return '{:%Y-%m-%d}'.format(date)


def ChronoNaiveTimeProvider(valobj, _dict):
    # type: (SBValue, dict) -> str
    origin_secs = valobj.GetChildMemberWithName("secs").GetValueAsUnsigned()
    nanos = valobj.GetChildMemberWithName("frac").GetValueAsUnsigned()
    (mins, sec) = div_mod_floor(origin_secs, 60)
    (hour, minutes) = div_mod_floor(mins, 60)
    (micro, _) = div_mod_floor(nanos, 1000)
    return '{:02d}:{:02d}:{:02d}.{:06d}Z'.format(hour, minutes, sec, micro)


def ChronoNaiveDateTimeProvider(valobj, _dict):
    # type: (SBValue, dict) -> str
    date = ChronoNaiveDateProvider(valobj.GetChildMemberWithName("date"), _dict)
    time = ChronoNaiveTimeProvider(valobj.GetChildMemberWithName("time"), _dict)
    return '\"{}T{}\"'.format(date, time)


def ChronoDateTimeProvider(valobj, _dict):
    # type: (SBValue, dict) -> str
    datetime = ChronoNaiveDateTimeProvider(valobj.GetChildMemberWithName("datetime"), _dict)
    return datetime