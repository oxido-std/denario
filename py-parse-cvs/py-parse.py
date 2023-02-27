
from pathlib import Path
from pathlib import Path, PureWindowsPath
import re

# 1. Abre el archivo
# 2. Lee línea por línea y lo formatea
# 3. El formato será de un insert grande a SQL
# 4. Guarda ese formato en un archivo SQL.

months=["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"]

# base = Path.cwd()
base2=PureWindowsPath("D:\\0_projects\\denario\\py-parse-cvs")

cvs_file=Path(base2,"excel-cvs.txt")
cvs_file=Path(cvs_file)
# archivo de salida
sql_file=Path(base2,"insert_massive_records.sql")
# print(cvs_file)

# Abro el archivo y leo línea por línea formatéandolas
cvs_file_opened=open(cvs_file,"r")


# recorro el archivo cvs y formateo cada línea y ese nuevo formato lo almaceno en una variable
for line in cvs_file_opened.readlines():
    # reemplazo todos los números que tengan d,d a esos les quito la ,
    regex_patter="(\"+[\d]*[,]+[\d]+\.[\d]+\")"
    # Busca por expresión regular los números que tengan , y la reemplaza por ''
    line=re.sub(regex_patter, lambda x: x.group(1).replace(',', ''), line)
    # quita las comillas dobles y esplitea la línea
    line_splited=line.rstrip().replace("\"","").split(",")
    # ---
    # tabla de records
    # name,amount,amount_io,comment,record_date,category_id,created_at,updated_at,is_deleted (9)
    # 
    # [2],[0/1], <-,[2],[5-6],999,DATETIME('NOW'),DATETIME('NOW'),false
    # 
    # ---
    
    # convierto meses a números
    for i, month in enumerate(months,1):
        if line_splited[5].lower()==month :
            if i < 10:
                i=str(i)
                i="0"+i
            line_splited[5]=str(i)
            break
    
    # print(sel_month)
    # creo la lista con los nuevos valores
    
    record=list(("name","amount","amount_io","comment","record_date","category_id","created_at","updated_at","is_deleted"))
    record[0]=line_splited[2] # name
    record[1]=line_splited[0] # por defecto es el valor 0
    amount_io="in" #amount_io
    
    if line_splited[0]=="":
        amount_io="out"
        record[1]=line_splited[1]
    record[2]=amount_io
    
    record[3]=line_splited[2] # comment
    record[4]=line_splited[6]+"-"+line_splited[5]+"-01" # date
    record[5]=999 #category
    record[6]="DATETIME('NOW')"
    record[7]="DATETIME('NOW')"
    record[8]="false"
    
    #mejoro el agregado de la categoría
    
    categories={
        "gas":11,
        "luz":10,
        "transporte": 17,
        "sube": 17,
        "teléfono": 16,
        "Teléfono": 16,
        "movistar": 16,
        "claro": 16,
        "tuenti": 16,
        "psicóloga": 19,
        "psiquiatra": 19,
        "internet": 12,
        "steam": 7,
        "juego": 7,
        "curso": 5,
        "taller": 5,
        "libro": 18,
        "verdulería": 3,
        "mercadito": 3,
        "monotributo": 21,
        "afip": 21,
        "adobe": 15,
        "crédito": 4,
        "credito": 4,
        "extra": 6,
    }
    
    # le agrego el código de la categoría
    name=record[0].lower()
    for key,val in categories.items():
        if key in name:
            record[5]=val
            break
        elif record[2]=="in" and (name=="abu" or "cambio" in name):
            record[5]=6
    
    
    
    invalid_names=["","SUBTOTAL","Excepcionales","Cuotas","Ahorro","Cierre del mes","SUBTOTAL de gastos fijos (nini este mes es poco)"]
    
    if record[0] not in invalid_names:
        sql_file_opened=open(sql_file,"a")
        record_str=str(record)
        record_str=record_str.replace("[","(")
        record_str=record_str.replace("]","),")
        record_str=record_str.replace("\"DATETIME('NOW')\"","DATETIME('NOW')")
        record_str=record_str.replace("\'false\'","false")
        sql_file_opened.write(record_str+"\n")
        sql_file_opened.close()

